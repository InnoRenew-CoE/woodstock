use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;
use anyhow::Result;
use serde_json::{json, Map};

pub trait Embeddable {
    fn try_into_embed(&self) -> GenerateEmbeddingsRequest;
    fn set_embedding_vectors(&mut self, embedding_vector: Vec<Vec<f32>>);
    fn prepare_for_upload(self, parent_doc_id: String) -> Result<Vec<EmbeddedChunk>>;
}

use qdrant_client::qdrant::PointStruct;
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

impl Into<PointStruct> for EmbeddedChunk {
    fn into(self) -> PointStruct {
        let mut payload = Map::new();
        payload.insert("doc_id".to_string(), Value::String(self.doc_id));
        payload.insert("doc_seq_num".to_string(), Value::Number(self.doc_seq_num.into()));
        payload.insert("content".to_string(), Value::String(self.content));
        payload.insert("additional_data".to_string(), self.additional_data);

        PointStruct::new(
            self.id,
            self.embedding_vector,
            payload,
        )
    }
}
