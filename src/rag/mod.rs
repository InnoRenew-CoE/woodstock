use comm::ollama::OllamaClient;
use anyhow::Result;
use loading::loader::load_file;
use crate::shared::file::WoodstockFileData;

mod comm;
mod models;
mod loading;

#[derive(Debug, Default)]
pub struct Rag {
    ollama: OllamaClient,
    vector_strore: (),
}


impl Rag {
    pub fn insert(&self, file: WoodstockFileData) -> Result<()>{
        let loaded_file = load_file(&file)?;
        println!("{:#?}", loaded_file);
        Ok(())
    }
}

