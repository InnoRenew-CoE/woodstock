use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use anyhow::Result;

pub trait Embeddable {
    fn try_into_embed(&self) -> GenerateEmbeddingsRequest;
    fn set_embedding_vectors(&mut self, embedding_vector: Vec<Vec<f32>>);
    fn prepare_for_upload(self, parent_doc_id: String) -> Result<Vec<EmbeddedChunk>>;
}

use serde_json::Value;

#[derive(Debug)]
pub struct EmbeddedChunk {
    pub embedding_vector: Vec<f32>,
    pub id: String,
    pub doc_id: String,
    pub doc_seq_num: i32,
    pub content: String,
    pub additional_data: Value,
}