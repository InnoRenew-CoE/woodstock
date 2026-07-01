use anyhow::{anyhow, Result};
use comm::embedding::EmbeddingVector;
use comm::qdrant::{insert_chunks_to_qdrant, vector_search};
use comm::{ChatClient, OllamaEmbeddingClient};
use loading::load_file;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use processing::{chunk, dedup, hype, prepare_for_upload};

pub mod agent;
pub mod comm;
pub mod loading;
pub mod models;
mod processing;

pub use loading::loaded_data::LoadedFile;
pub use models::chunks::{Chunk, EmbeddedChunk, HypeChunk, ResultChunk};
pub use models::ChunkedFile;
pub use models::{RagProcessableFile, RagProcessableFileType};

#[derive(Debug)]
pub struct Rag {
    pub llm: ChatClient,
    pub embeddings: OllamaEmbeddingClient,
}

impl Default for Rag {
    fn default() -> Self {
        Self {
            llm: ChatClient::default(),
            embeddings: OllamaEmbeddingClient::default(),
        }
    }
}

impl Rag {
    pub fn for_ingestion() -> Self {
        Self {
            llm: ChatClient::for_ingestion(),
            embeddings: OllamaEmbeddingClient::for_ingestion(),
        }
    }
    pub async fn insert(&self, file: RagProcessableFile) -> Result<()> {
        let loaded_file = self.insert_meta(&file).await?;
        let chunked_file = Self::insert_chunk(loaded_file);
        let hyped_file = self.insert_hype(chunked_file).await?;
        let embedded_chunks = self.insert_embed(hyped_file).await?;
        Self::insert_qdrant(embedded_chunks).await
    }

    pub async fn insert_meta(&self, file: &RagProcessableFile) -> Result<LoadedFile> {
        load_file(file).await
    }

    pub fn insert_chunk(loaded: LoadedFile) -> ChunkedFile<Chunk> {
        chunk(loaded, processing::ChunkingStrategy::Markdown(250))
    }

    pub async fn insert_hype(&self, chunked: ChunkedFile<Chunk>) -> Result<ChunkedFile<HypeChunk>> {
        Ok(hype(chunked, &self.llm).await)
    }

    pub async fn insert_embed(&self, hyped: ChunkedFile<HypeChunk>) -> Result<Vec<EmbeddedChunk>> {
        prepare_for_upload(hyped, &self.embeddings).await
    }

    pub async fn insert_qdrant(embedded_chunks: Vec<EmbeddedChunk>) -> Result<()> {
        insert_chunks_to_qdrant(embedded_chunks).await
    }

    pub async fn search_raw(&self, query: String) -> Result<Vec<ResultChunk>> {
        let emb_query = GenerateEmbeddingsRequest::new("bge-m3".to_owned(), EmbeddingsInput::Single(query.clone()));
        let embedding = match self.embeddings.embed(emb_query).await {
            Ok(resp) => EmbeddingVector(resp.embeddings[0].clone()),
            Err(e) => return Err(anyhow!(format!("Failed embedding the query: {}", e))),
        };
        let resp = vector_search(embedding).await?;
        let resp = dedup(resp);
        println!("{:#?}", resp);
        Ok(resp)
    }
}
