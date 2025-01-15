use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use anyhow::{Result, anyhow};
use serde_json::Value;
use crate::rag::comm::embedding::{Embeddable, EmbeddedChunk};


#[derive(Debug)]
pub struct Chunk {
    pub seq_num: i32,
    pub text: String,
    pub embedding_vector: Option<Vec<f32>>,
}

impl Embeddable for Chunk {
    fn try_into_embed(&self) -> GenerateEmbeddingsRequest {
        GenerateEmbeddingsRequest::new(
            "bge-m3".to_owned(),
            EmbeddingsInput::Single(self.text.clone())
        )
    }
    
    fn set_embedding_vectors(&mut self, embedding_vectors: Vec<Vec<f32>>) {
        self.embedding_vector = Some(embedding_vectors[0].clone());
    }
    
    fn prepare_for_upload(self, doc_id: String) -> Result<Vec<EmbeddedChunk>> {
        let embedding_vector = match self.embedding_vector {
            Some(v) => v,
            None => return Err(anyhow!("No embedding vector on chunk")),
        };
        Ok(vec![EmbeddedChunk {
            embedding_vector,
            id: uuid::Uuid::new_v4().to_string(),
            doc_id,
            doc_seq_num: self.seq_num,
            content: self.text,
            additional_data: Value::Null,
        }])
    }

    

}