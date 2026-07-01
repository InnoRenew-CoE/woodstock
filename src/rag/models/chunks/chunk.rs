use crate::rag::{
    comm::embedding::{Embeddable, EmbeddingVector},
    models::ImageRef,
};
use anyhow::{anyhow, Result};
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::embedded_chunk::EmbeddedChunk;

pub fn image_payload(images: &[ImageRef]) -> serde_json::Value {
    serde_json::Value::Array(
        images
            .iter()
            .map(|image| {
                json!({
                    "id": image.id,
                    "document_id": image.document_id,
                    "file_name": image.file_name,
                    "route": image.route,
                    "alt_text": image.alt_text,
                })
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub seq_num: i32,
    pub text: String,
    pub embedding_vector: Option<EmbeddingVector>,
    #[serde(default)]
    pub images: Vec<ImageRef>,
}

impl Embeddable for Chunk {
    fn try_into_embed(&self) -> GenerateEmbeddingsRequest {
        GenerateEmbeddingsRequest::new("bge-m3".to_owned(), EmbeddingsInput::Single(self.text.clone()))
    }

    fn set_embedding_vectors(&mut self, embedding_vectors: Vec<EmbeddingVector>) {
        self.embedding_vector = Some(embedding_vectors[0].clone());
    }

    fn prepare_for_upload(self, doc_id: String, doc_summary: Option<String>) -> Result<Vec<EmbeddedChunk>> {
        let embedding_vector = match self.embedding_vector {
            Some(v) => v,
            None => return Err(anyhow!("No embedding vector on chunk")),
        };
        let doc_summary = if let Some(summ) = doc_summary { summ } else { "".to_string() };
        Ok(vec![EmbeddedChunk {
            embedding_vector,
            id: uuid::Uuid::new_v4().to_string(),
            doc_id,
            doc_seq_num: self.seq_num,
            content: self.text,
            additional_data: json!({
                "images": image_payload(&self.images),
            }),
            doc_summary,
        }])
    }
}
