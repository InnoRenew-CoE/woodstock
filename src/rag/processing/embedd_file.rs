use crate::rag::{comm::{embedding::{Embeddable, EmbeddingVector}, OllamaClient}, models::ChunkedFile};
use anyhow::{Result, anyhow};
use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

pub async fn embedd_file<T>(mut file: ChunkedFile<T>, ollama: &OllamaClient) -> Result<ChunkedFile<T>> where T: Embeddable {
    let requests: Vec<GenerateEmbeddingsRequest> = file
        .chunks
        .iter()
        .map(|c| c.try_into_embed())
        .collect();

    
    let all_embeddings = embedd_all(requests, &ollama).await;
    
    if file.chunks.len() != all_embeddings.len() {
        return Err(anyhow!("Not all embeddings were successful."));
    }

    let chunks_with_embeddings: Vec<(&mut T, Vec<EmbeddingVector>)> = file
        .chunks
        .iter_mut()
        .zip(all_embeddings.into_iter())
        .collect();

    for (chunk, embeddings) in chunks_with_embeddings {
        chunk.set_embedding_vectors(embeddings);
    }

    Ok(file)
}


async fn embedd_all(requests: Vec<GenerateEmbeddingsRequest>, ollama: &OllamaClient) -> Vec<Vec<EmbeddingVector>> {
    let futures = requests.into_iter().map(|r| async move {
        ollama.embed(r).await.ok()
    });

    let results = futures::future::join_all(futures).await;
    results.into_iter()
        .filter_map(|resp|  match resp { 
            Some(r) => Some(r
                .embeddings
                .into_iter()
                .map(|e| EmbeddingVector(e)).collect()),
            None => None
         })
        .collect()
}
