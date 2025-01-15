use chunking::{chunk, hype_chunk::hype, prepare::prepare_for_upload};
use comm::{embedding::EmbeddingVector, qdrant::{insert_chunks_to_qdrant, vector_search}, OllamaClient};
use anyhow::{Result, anyhow};
use loading::load_file;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
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


    pub async fn search(&self, query: String) -> Result<()> {
        let emb_query = GenerateEmbeddingsRequest::new(
            "bge-m3".to_owned(), 
            EmbeddingsInput::Single(query)
        );
        let embedding = match self.ollama.embed(emb_query).await {
            Ok(resp) => EmbeddingVector(resp.embeddings[0].clone()),
            Err(e) => return Err(anyhow!(format!("Failed embedding the query: {}", e))),
        };
        let resp = vector_search(embedding).await?;
        println!("{:#?}", resp);
        Ok(())
    }
}

