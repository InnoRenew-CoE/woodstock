use crate::rag::{
    comm::{embedding::Embeddable, OllamaClient}, 
    models::{chunks::EmbeddedChunk, ChunkedFile}
};
use anyhow::Result;

use super::embedd_file::embedd_file;


pub async fn prepare_for_upload<T>(file: ChunkedFile<T>, ollama: &OllamaClient) -> Result<Vec<EmbeddedChunk>> where T: Embeddable {
    let descr = file.syntetic_file_description.clone();
    let embedded_file = embedd_file(file, ollama).await?;
    Ok(embedded_file
        .chunks
        .into_iter()
        .filter_map(|c| c.prepare_for_upload(embedded_file.internal_id.to_string(), descr.clone()).ok())
        .flatten()
        .collect())
}