use chunking::{chunk, hype_chunk::hype, prepare::prepare_for_upload};
use comm::{qdrant::insert_chunks_to_qdrant, OllamaClient};
use anyhow::Result;
use loading::load_file;
use crate::shared::file::WoodstockFileData;

pub mod comm;
mod loading;
mod chunking;

#[derive(Debug, Default)]
pub struct Rag {
    ollama: OllamaClient,
}


impl Rag {
    pub async fn insert(&self, file: WoodstockFileData) -> Result<()>{
        let loaded_file = load_file(&file)?;
        let chunked_file = chunk(loaded_file, chunking::ChunkingStrategy::Word(250, 30));
        let enriched_file = hype(chunked_file, &self.ollama).await;
        let embedded_chunks = prepare_for_upload(enriched_file, &self.ollama).await?;
        insert_chunks_to_qdrant(embedded_chunks).await
    }
}

