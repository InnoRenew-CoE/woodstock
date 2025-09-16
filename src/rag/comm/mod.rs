use futures::{stream::{self, FuturesUnordered}, StreamExt};
use ollama_rs::{
    error::OllamaError,
    generation::{
        completion::{GenerationResponse, GenerationResponseStream},
        embeddings::{request::GenerateEmbeddingsRequest, GenerateEmbeddingsResponse},
    },
    Ollama,
};
use question::Question;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Semaphore;
use std::{env, future::Future, sync::Arc, time::Duration};
use ollama_rs::error::OllamaError::*;
use rand::{rng, thread_rng, Rng};

pub mod embedding;
pub mod qdrant;
pub mod question;
pub mod marker;



#[derive(Debug, Clone)]
pub struct BackoffConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub factor: f64,
    pub max_delay: Duration,
    pub jitter_ratio: f64, // 0.0 means no jitter, 0.5 means add up to 50 percent random extra
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            max_retries: 7,
            initial_delay: Duration::from_secs(5),
            factor: 2.0,
            max_delay: Duration::from_secs(1800),
            jitter_ratio: 0.3,
        }
    }
}


#[derive(Debug)]
pub struct OllamaClient {
    ollama: Ollama,
    backoff: BackoffConfig,
}

impl Default for OllamaClient {
    fn default() -> Self {
        let ollama_host = env::var("OLLAMA_HOST").expect("OLLAMA HOST not set");
        let ollama_port = env::var("OLLAMA_PORT").expect("OLLAMA PORT not set");
        let ollama_port: u16 = ollama_port.parse().expect("OLLAMA_PORT not u16");

        Self {
            ollama: Ollama::new(ollama_host, ollama_port),
            backoff: BackoffConfig::default(),
        }
    }
}

impl OllamaClient {
    pub fn with_backoff(mut self, backoff: BackoffConfig) -> Self {
        self.backoff = backoff;
        self
    }

    pub async fn generate(&self, question: Question) -> Result<GenerationResponse, OllamaError> {
        self.ollama.generate((&question).into()).await
    }

    pub async fn generate_stream(&self, question: Question) -> Result<GenerationResponseStream, OllamaError> {
        self.ollama.generate_stream((&question).into()).await
    }

    pub async fn embed(&self, req: GenerateEmbeddingsRequest) -> Result<GenerateEmbeddingsResponse, OllamaError> {
        self.ollama.generate_embeddings(req).await
    }

    pub async fn answer_all(&self, questions: Vec<Question>) -> Vec<String> {
        let futures = questions.into_iter().map(|q| async move { self.generate(q.clone()).await.ok() });

        let results = futures::future::join_all(futures).await;
        results
            .into_iter()
            .map(|r| r.map_or_else(|| "".to_owned(), |resp| resp.response))
            .collect()
    }


    fn is_retryable(err: &OllamaError) -> bool {

        match err {
            // Network and HTTP conditions that are usually transient
            ReqwestError(e) => {
                if e.is_timeout() || e.is_connect() {
                    return true;
                }
                if let Some(status) = e.status() {
                    return status.is_server_error() || status.as_u16() == 429;
                }
                // Other transport-layer issues without status can still be transient
                // For example: connection reset by peer before status is known
                let s = e.to_string().to_ascii_lowercase();
                return s.contains("connection reset")
                    || s.contains("broken pipe")
                    || s.contains("connection refused")
                    || s.contains("timed out");
            }

            // Ollama surfaced an internal condition. Retry if message smells transient.
            InternalError(e) => {
                let msg = e.message.to_ascii_lowercase();
                return msg.contains("unavailable")
                    || msg.contains("busy")
                    || msg.contains("loading")
                    || msg.contains("not ready")
                    || msg.contains("temporar")          // temporary, temporarily
                    || msg.contains("timeout")
                    || msg.contains("rate limit")
                    || msg.contains("too many requests")
                    || msg.contains("model is downloading")
                    || msg.contains("no available slots")
                    || msg.contains("queue")
                    || msg.contains("connection reset")
                    || msg.contains("broken pipe");
            }

            // Deserialization problems are usually deterministic inputs, so do not retry by default.
            JsonError(_) => false,

            // Tool call errors are generally logic issues in the tool itself.
            ToolCallError(_) => false,

            // Unknown bucket, be conservative and retry
            Other(_) => true,
        }
    }


    async fn retry_with_backoff<F, Fut, T>(&self, mut op: F) -> Result<T, OllamaError>
    where
        F: FnMut(u32) -> Fut,
        Fut: Future<Output = Result<T, OllamaError>>,
    {
        let cfg = &self.backoff;
        let mut attempt: u32 = 0;
        let mut delay = cfg.initial_delay;

        loop {
            match op(attempt).await {
                Ok(val) => return Ok(val),
                Err(err) => {
                    if attempt >= cfg.max_retries || !Self::is_retryable(&err) {
                        return Err(err);
                    }
                    let mut rng = rng();
                    let jitter_ns = (delay.as_nanos() as f64 * cfg.jitter_ratio * rng.gen::<f64>()) as u128;
                    let next_delay = delay
                        .saturating_add(Duration::from_nanos(jitter_ns as u64))
                        .min(cfg.max_delay);

                    tokio::time::sleep(next_delay).await;

                    // increase delay for next round
                    let next_ms = (delay.as_millis() as f64 * cfg.factor) as u64;
                    delay = Duration::from_millis(next_ms).min(cfg.max_delay);
                    attempt += 1;
                }
            }
        }
    }

    // Public retrying variants
    pub async fn generate_with_retry(&self, question: Question) -> Result<GenerationResponse, OllamaError> {
        self.retry_with_backoff(|_attempt| async {
            self.ollama.generate((&question).into()).await
        }).await
    }

    pub async fn generate_stream_with_retry(&self, question: Question) -> Result<GenerationResponseStream, OllamaError> {
        // Note: if the stream fails partway through, the caller must decide how to handle it
        self.retry_with_backoff(|_attempt| async {
            self.ollama.generate_stream((&question).into()).await
        }).await
    }

    pub async fn answer_all_with_retry(
        &self, 
        questions: Vec<Question>, 
        concurrency: usize
    ) -> Vec<String> {
        let n = questions.len();
        let sem = Arc::new(Semaphore::new(concurrency.max(1)));
        let mut futs = FuturesUnordered::new();
        let base_client = self.ollama.clone();
        let base_backoff = self.backoff.clone();

        for (idx, q) in questions.into_iter().enumerate() {
            let sem = sem.clone();
            let client = base_client.clone();
            let backoff = base_backoff.clone();

            futs.push(async move {
                let _permit = sem.acquire().await.expect("semaphore");
                let text = generate_with_retry(client, backoff, q)
                    .await
                    .map(|r| r.response)
                    .unwrap_or_default();
                (idx, text)
            });
        }


        // gather, preserving input order
        let mut out = vec![String::new(); n];
        while let Some((idx, text)) = futs.next().await {
            out[idx] = text;
        }
        out
    }
}

async fn generate_with_retry(
    ollama: Ollama,
    backoff: BackoffConfig,
    question: Question,
) -> Result<GenerationResponse, OllamaError> {
    let mut attempt = 0u32;
    let mut delay = backoff.initial_delay;

    loop {
        match ollama.generate((&question).into()).await {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                
                if attempt >= backoff.max_retries || !OllamaClient::is_retryable(&err) {
                    return Err(err);
                }

                let next_ms = ((delay.as_millis() as f64) * backoff.factor) as u64;
                delay = Duration::from_millis(next_ms).min(backoff.max_delay);
                attempt += 1;
            }
        }
    }
}