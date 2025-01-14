use chunking::{chunk, hype_chunk::{self, hype}};
use comm::OllamaClient;
use anyhow::Result;
use loading::load_file;
use crate::shared::file::WoodstockFileData;

pub mod comm;
mod models;
mod loading;
mod chunking;

#[derive(Debug, Default)]
pub struct Rag {
    ollama: OllamaClient,
    vector_strore: (),
}


impl Rag {
    pub async fn insert(&self, file: WoodstockFileData) -> Result<()>{
        let loaded_file = load_file(&file)?;
        let chunked_file = chunk(loaded_file, chunking::ChunkingStrategy::Word(250, 30));
        let enriched_file = hype(chunked_file, &self.ollama).await;
        println!("{:#?}", enriched_file);
        Ok(())
    }
}

