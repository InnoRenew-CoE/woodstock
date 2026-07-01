use anyhow::{anyhow, Result};
use futures::{stream, Stream, StreamExt};
use ollama_rs::error::OllamaError;
use ollama_rs::error::OllamaError::*;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use ollama_rs::generation::embeddings::GenerateEmbeddingsResponse;
use ollama_rs::Ollama;
use question::Question;
use rand::{rng, Rng};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod embedding;
pub mod marker;
pub mod qdrant;
pub mod question;

pub type ChatTextStream = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub content: String,
}

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
pub struct OllamaEmbeddingClient {
    ollama: Ollama,
}

impl OllamaEmbeddingClient {
    fn from_env(host_key: &str, port_key: &str) -> Self {
        let ollama_host = env::var(host_key).expect(&format!("{host_key} not set"));
        let ollama_port = env::var(port_key).expect(&format!("{port_key} not set"));
        let ollama_port: u16 = ollama_port.parse().expect(&format!("{port_key} not u16"));
        let api_key = env::var("API_KEY").ok();
        let mut ollama = Ollama::new(ollama_host, ollama_port);

        if let Some(api_key) = api_key {
            let mut headers = HeaderMap::new();
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
            ollama.set_headers(Some(headers));
        }

        Self { ollama }
    }
}

impl OllamaEmbeddingClient {
    pub fn for_ingestion() -> Self {
        Self::from_env("INGESTION_OLLAMA_HOST", "INGESTION_OLLAMA_PORT")
    }
}

impl Default for OllamaEmbeddingClient {
    fn default() -> Self {
        Self::from_env("OLLAMA_HOST", "OLLAMA_PORT")
    }
}

impl OllamaEmbeddingClient {
    pub async fn embed(&self, req: GenerateEmbeddingsRequest) -> Result<GenerateEmbeddingsResponse, OllamaError> {
        self.ollama.generate_embeddings(req).await
    }
}

#[derive(Debug)]
pub enum ChatClient {
    Ollama(OllamaChatClient),
    OpenAICompatible(OpenAICompatibleChatClient),
}

impl ChatClient {
    fn from_provider(provider_key: &str, ollama_host_key: &str, ollama_port_key: &str, openai_prefix: &str) -> Self {
        match env::var(provider_key).unwrap_or_else(|_| "openai-compatible".to_owned()).as_str() {
            "ollama" => Self::Ollama(OllamaChatClient::from_env(ollama_host_key, ollama_port_key)),
            "openai-compatible" | "oaic" | "openai" => Self::OpenAICompatible(OpenAICompatibleChatClient::from_prefix(openai_prefix)),
            other => panic!("Unsupported {provider_key}: {other}"),
        }
    }

    pub fn for_ingestion() -> Self {
        Self::from_provider("INGESTION_CHAT_PROVIDER", "INGESTION_OLLAMA_HOST", "INGESTION_OLLAMA_PORT", "INGESTION")
    }
}

impl Default for ChatClient {
    fn default() -> Self {
        Self::from_provider("CHAT_PROVIDER", "OLLAMA_HOST", "OLLAMA_PORT", "OPENAI")
    }
}

impl ChatClient {
    pub fn with_backoff(mut self, backoff: BackoffConfig) -> Self {
        match &mut self {
            Self::Ollama(client) => client.backoff = backoff,
            Self::OpenAICompatible(client) => client.backoff = backoff,
        }
        self
    }

    pub async fn generate(&self, question: Question) -> Result<ChatResponse> {
        match self {
            Self::Ollama(client) => client.generate(question).await,
            Self::OpenAICompatible(client) => client.generate(question).await,
        }
    }

    pub async fn generate_stream(&self, question: Question) -> Result<ChatTextStream> {
        match self {
            Self::Ollama(client) => client.generate_stream(question).await,
            Self::OpenAICompatible(client) => client.generate_stream(question).await,
        }
    }

    pub async fn answer_all(&self, questions: Vec<Question>) -> Vec<String> {
        let futures = questions.into_iter().map(|q| async move { self.generate(q).await.ok() });

        let results = futures::future::join_all(futures).await;
        results
            .into_iter()
            .map(|r| r.map_or_else(|| "".to_owned(), |resp| resp.content))
            .collect()
    }

    pub async fn generate_with_retry(&self, question: Question) -> Result<ChatResponse> {
        match self {
            Self::Ollama(client) => client.generate_with_retry(question).await,
            Self::OpenAICompatible(client) => client.generate_with_retry(question).await,
        }
    }

    pub async fn generate_stream_with_retry(&self, question: Question) -> Result<ChatTextStream> {
        match self {
            Self::Ollama(client) => client.generate_stream_with_retry(question).await,
            Self::OpenAICompatible(client) => client.generate_stream_with_retry(question).await,
        }
    }

    pub async fn answer_all_with_retry(&self, questions: Vec<Question>, concurrency: usize) -> Vec<String> {
        stream::iter(
            questions
                .into_iter()
                .map(|q| async move { self.generate_with_retry(q).await.map(|resp| resp.content).unwrap_or_default() }),
        )
        .buffer_unordered(concurrency.max(1))
        .collect::<Vec<_>>()
        .await
    }
}

#[derive(Debug)]
pub struct OllamaChatClient {
    ollama: Ollama,
    backoff: BackoffConfig,
}

impl OllamaChatClient {
    fn from_env(host_key: &str, port_key: &str) -> Self {
        let ollama_host = env::var(host_key).expect(&format!("{host_key} not set"));
        let ollama_port = env::var(port_key).expect(&format!("{port_key} not set"));
        let ollama_port: u16 = ollama_port.parse().expect(&format!("{port_key} not u16"));
        let api_key = env::var("API_KEY").ok();
        let mut ollama = Ollama::new(ollama_host, ollama_port);

        if let Some(api_key) = api_key {
            let mut headers = HeaderMap::new();
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
            ollama.set_headers(Some(headers));
        }

        Self {
            ollama,
            backoff: BackoffConfig::default(),
        }
    }
}

impl Default for OllamaChatClient {
    fn default() -> Self {
        Self::from_env("OLLAMA_HOST", "OLLAMA_PORT")
    }
}

impl OllamaChatClient {
    pub async fn generate(&self, question: Question) -> Result<ChatResponse> {
        let resp = self.ollama.generate(question_to_ollama_request(&question)).await?;
        Ok(ChatResponse { content: resp.response })
    }

    pub async fn generate_stream(&self, question: Question) -> Result<ChatTextStream> {
        let stream = self.ollama.generate_stream(question_to_ollama_request(&question)).await?.map(|res| {
            res.map(|responses| responses.into_iter().map(|resp| resp.response).collect::<String>())
                .map_err(|err| anyhow!(err.to_string()))
        });

        Ok(Box::pin(stream))
    }

    async fn retry_with_backoff<F, Fut, T>(&self, mut op: F) -> Result<T>
    where
        F: FnMut(u32) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        retry_with_backoff(&self.backoff, |attempt| op(attempt)).await
    }

    pub async fn generate_with_retry(&self, question: Question) -> Result<ChatResponse> {
        self.retry_with_backoff(|_attempt| async { self.generate(question.clone()).await }).await
    }

    pub async fn generate_stream_with_retry(&self, question: Question) -> Result<ChatTextStream> {
        self.retry_with_backoff(|_attempt| async { self.generate_stream(question.clone()).await })
            .await
    }
}

#[derive(Debug)]
pub struct OpenAICompatibleChatClient {
    http: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    model: String,
    backoff: BackoffConfig,
}

impl OpenAICompatibleChatClient {
    fn from_prefix(prefix: &str) -> Self {
        let base_url_key = format!("{prefix}_COMPATIBLE_BASE_URL");
        let api_key_key = format!("{prefix}_COMPATIBLE_API_KEY");
        let model_key = format!("{prefix}_MODEL");
        Self {
            http: reqwest::Client::new(),
            base_url: env::var(&base_url_key)
                .unwrap_or_else(|_| env::var("OPENAI_COMPATIBLE_BASE_URL")
                    .or_else(|_| env::var("OPENAI_BASE_URL"))
                    .unwrap_or_else(|_| "http://localhost:11434/v1".to_owned()))
                .trim_end_matches('/')
                .to_owned(),
            api_key: env::var(&api_key_key)
                .or_else(|_| env::var("OPENAI_COMPATIBLE_API_KEY"))
                .or_else(|_| env::var("OPENAI_API_KEY"))
                .or_else(|_| env::var("API_KEY"))
                .ok(),
            model: env::var(&model_key).unwrap_or_else(|_|
                env::var("CHAT_MODEL").unwrap_or_else(|_|
                    "hf.co/unsloth/Qwen3-30B-A3B-Instruct-2507-GGUF:UD-Q4_K_XL".to_owned())),
            backoff: BackoffConfig::default(),
        }
    }
}

impl Default for OpenAICompatibleChatClient {
    fn default() -> Self {
        Self::from_prefix("OPENAI")
    }
}

impl OpenAICompatibleChatClient {
    pub async fn generate(&self, question: Question) -> Result<ChatResponse> {
        let request = OpenAIChatRequest {
            model: self.model_for(&question),
            messages: question_to_openai_messages(&question),
            stream: false,
        };

        let response = self
            .request_builder()
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<OpenAIChatResponse>()
            .await?;

        let content = response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .unwrap_or_default();

        Ok(ChatResponse { content })
    }

    pub async fn generate_stream(&self, question: Question) -> Result<ChatTextStream> {
        let request = OpenAIChatRequest {
            model: self.model_for(&question),
            messages: question_to_openai_messages(&question),
            stream: true,
        };

        let response = self.request_builder().json(&request).send().await?.error_for_status()?;
        let mut bytes = response.bytes_stream();
        let (tx, rx) = mpsc::channel::<Result<String>>(1024);

        tokio::spawn(async move {
            let mut buffer = String::new();

            while let Some(chunk) = bytes.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(err) => {
                        let _ = tx.send(Err(anyhow!(err.to_string()))).await;
                        return;
                    }
                };

                buffer.push_str(&String::from_utf8_lossy(&chunk));

                while let Some(newline_idx) = buffer.find('\n') {
                    let line = buffer[..newline_idx].trim().to_owned();
                    buffer.drain(..=newline_idx);

                    let Some(data) = line.strip_prefix("data:") else {
                        continue;
                    };
                    let data = data.trim();
                    if data == "[DONE]" {
                        return;
                    }
                    if data.is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<OpenAIChatStreamResponse>(data) {
                        Ok(response) => {
                            for choice in response.choices {
                                if let Some(content) = choice.delta.content {
                                    if !content.is_empty() && tx.send(Ok(content)).await.is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            let _ = tx.send(Err(anyhow!(err.to_string()))).await;
                            return;
                        }
                    }
                }
            }
        });

        Ok(Box::pin(ReceiverStream::new(rx)))
    }

    async fn retry_with_backoff<F, Fut, T>(&self, mut op: F) -> Result<T>
    where
        F: FnMut(u32) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        retry_with_backoff(&self.backoff, |attempt| op(attempt)).await
    }

    pub async fn generate_with_retry(&self, question: Question) -> Result<ChatResponse> {
        self.retry_with_backoff(|_attempt| async { self.generate(question.clone()).await }).await
    }

    pub async fn generate_stream_with_retry(&self, question: Question) -> Result<ChatTextStream> {
        self.retry_with_backoff(|_attempt| async { self.generate_stream(question.clone()).await })
            .await
    }

    fn request_builder(&self) -> reqwest::RequestBuilder {
        let builder = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .header(CONTENT_TYPE, "application/json");

        if let Some(api_key) = &self.api_key {
            builder.header(AUTHORIZATION, format!("Bearer {}", api_key))
        } else {
            builder
        }
    }

    fn model_for(&self, _question: &Question) -> String {
        self.model.clone()
    }
}

fn question_to_ollama_request(question: &Question) -> GenerationRequest<'static> {
    let final_prompt = format!("{}\n{}", question.system_prompt(), question.user_content());
    GenerationRequest::new(question.model().to_owned(), final_prompt)
}

fn question_to_openai_messages(question: &Question) -> Vec<OpenAIChatMessage> {
    vec![
        OpenAIChatMessage {
            role: "system",
            content: question.system_prompt().to_owned(),
        },
        OpenAIChatMessage {
            role: "user",
            content: question.user_content(),
        },
    ]
}

fn is_retryable(err: &anyhow::Error) -> bool {
    if let Some(err) = err.downcast_ref::<OllamaError>() {
        return is_retryable_ollama(err);
    }

    if let Some(err) = err.downcast_ref::<reqwest::Error>() {
        if err.is_timeout() || err.is_connect() {
            return true;
        }
        if let Some(status) = err.status() {
            return status.is_server_error() || status.as_u16() == 429;
        }
        let s = err.to_string().to_ascii_lowercase();
        return s.contains("connection reset") || s.contains("broken pipe") || s.contains("connection refused") || s.contains("timed out");
    }

    true
}

fn is_retryable_ollama(err: &OllamaError) -> bool {
    match err {
        ReqwestError(e) => {
            if e.is_timeout() || e.is_connect() {
                return true;
            }
            if let Some(status) = e.status() {
                return status.is_server_error() || status.as_u16() == 429;
            }
            let s = e.to_string().to_ascii_lowercase();
            s.contains("connection reset") || s.contains("broken pipe") || s.contains("connection refused") || s.contains("timed out")
        }
        InternalError(e) => {
            let msg = e.message.to_ascii_lowercase();
            msg.contains("unavailable")
                || msg.contains("busy")
                || msg.contains("loading")
                || msg.contains("not ready")
                || msg.contains("temporar")
                || msg.contains("timeout")
                || msg.contains("rate limit")
                || msg.contains("too many requests")
                || msg.contains("model is downloading")
                || msg.contains("no available slots")
                || msg.contains("queue")
                || msg.contains("connection reset")
                || msg.contains("broken pipe")
        }
        JsonError(_) => false,
        ToolCallError(_) => false,
        Other(_) => true,
    }
}

async fn retry_with_backoff<F, Fut, T>(cfg: &BackoffConfig, mut op: F) -> Result<T>
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempt: u32 = 0;
    let mut delay = cfg.initial_delay;

    loop {
        match op(attempt).await {
            Ok(val) => return Ok(val),
            Err(err) => {
                if attempt >= cfg.max_retries || !is_retryable(&err) {
                    return Err(err);
                }
                let mut rng = rng();
                let jitter_ns = (delay.as_nanos() as f64 * cfg.jitter_ratio * rng.gen::<f64>()) as u128;
                let next_delay = delay.saturating_add(Duration::from_nanos(jitter_ns as u64)).min(cfg.max_delay);

                tokio::time::sleep(next_delay).await;

                let next_ms = (delay.as_millis() as f64 * cfg.factor) as u64;
                delay = Duration::from_millis(next_ms).min(cfg.max_delay);
                attempt += 1;
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OpenAIChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChatChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatChoice {
    message: OpenAIChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatStreamResponse {
    choices: Vec<OpenAIChatStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatStreamChoice {
    delta: OpenAIChatStreamDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatStreamDelta {
    content: Option<String>,
}
