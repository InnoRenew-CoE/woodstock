use crate::rag::comm::{embedding::{Embeddable, EmbeddedChunk}, OllamaClient};
use anyhow::Result;
use super::{chunked_file::ChunkedFile, embedd_file::embedd_file};


pub async fn prepare_for_upload<T>(file: ChunkedFile<T>, ollama: &OllamaClient) -> Result<Vec<EmbeddedChunk>> where T: Embeddable {
    let embedded_file = embedd_file(file, ollama).await?;
    Ok(embedded_file
        .chunks
        .into_iter()
        .filter_map(|c| c.prepare_for_upload(embedded_file.internal_id.to_string()).ok())
        .flatten()
        .collect())
}