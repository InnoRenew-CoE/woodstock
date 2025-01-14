use chunking::chunk;
use comm::OllamaClient;
use anyhow::Result;
use loading::load_file;
use crate::shared::file::WoodstockFileData;

mod comm;
mod models;
mod loading;
mod chunking;

#[derive(Debug, Default)]
pub struct Rag {
    ollama: OllamaClient,
    vector_strore: (),
}


impl Rag {
    pub fn insert(&self, file: WoodstockFileData) -> Result<()>{
        let loaded_file = load_file(&file)?;
        let chunked_file = chunk(loaded_file, chunking::ChunkingStrategy::Word(250, 30));
        println!("{:#?}", chunked_file);
        Ok(())
    }
}

