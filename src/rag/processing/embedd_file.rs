use crate::rag::{
    comm::{
        embedding::{Embeddable, EmbeddingVector},
        OllamaEmbeddingClient,
    },
    models::ChunkedFile,
};
use anyhow::{anyhow, Result};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rand::{thread_rng, Rng};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Semaphore, time::sleep};

const MAX_ATTEMPTS: usize = 4;
const BASE_DELAY_MS: u64 = 1000;
const MAX_CONCURRENCY: usize = 64;

pub async fn embedd_file<T>(mut file: ChunkedFile<T>, embeddings: &OllamaEmbeddingClient) -> Result<ChunkedFile<T>>
where
    T: Embeddable + Clone,
{
    let sem = Arc::new(Semaphore::new(MAX_CONCURRENCY));
    let mut futs = FuturesUnordered::new();
    let chunk_count = file.chunks.len();
    for mut c in file.chunks.into_iter() {
        let sem = sem.clone();
        futs.push(async move {
            let _permit = sem.acquire().await.expect("semaphore");
            match embedd_questions(&mut c, embeddings).await {
                Ok(_) => Some(c),
                Err(error) => {
                    eprintln!("[EMBED] chunk dropped: {error}");
                    None
                }
            }
        });
    }

    let mut kept = Vec::new();
    while let Some(opt) = futs.next().await {
        if let Some(c) = opt {
            kept.push(c);
        }
    }

    if kept.len() < chunk_count {
        eprintln!("[EMBED] kept {} of {chunk_count} chunk(s)", kept.len());
    }
    file.chunks = kept;
    Ok(file)
}

async fn embedd_questions<T>(chunk: &mut T, client: &OllamaEmbeddingClient) -> Result<()>
where
    T: Embeddable + Clone,
{
    let mut last_error = String::new();
    for _ in 0..MAX_ATTEMPTS {
        let req = chunk.clone().try_into_embed();
        match client.embed(req).await {
            Ok(resp) => {
                let vectors = resp.embeddings.into_iter().map(EmbeddingVector).collect::<Vec<_>>();
                chunk.set_embedding_vectors(vectors);
                return Ok(());
            }
            Err(err) => {
                last_error = err.to_string();
                let jitter = jitter_ms(BASE_DELAY_MS / 2);
                sleep(Duration::from_millis(BASE_DELAY_MS + jitter)).await;
            }
        }
    }
    Err(anyhow!("could not embed chunk after {MAX_ATTEMPTS} attempts: {last_error}"))
}

fn jitter_ms(max: u64) -> u64 {
    if max == 0 {
        0
    } else {
        thread_rng().gen_range(0..=max)
    }
}
