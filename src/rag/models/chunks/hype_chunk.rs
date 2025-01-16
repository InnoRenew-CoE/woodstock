use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use regex::RegexBuilder;
use anyhow::{Result, anyhow};
use serde_json::Value;
use crate::rag::comm::{embedding::{Embeddable, EmbeddingVector}, question::Question, OllamaClient};

use super::{chunk::Chunk, embedded_chunk::EmbeddedChunk};

#[derive(Debug)]
pub struct HypeChunk {
    pub seq_num: i32,
    pub text: String,
    pub questions: Vec<String>,
    pub embedding_vector: Option<Vec<EmbeddingVector>>,
}

impl From<&Chunk> for HypeChunk {
    fn from(value: &Chunk) -> Self {
        Self { 
            seq_num: value.seq_num, 
            text: value.text.clone(), 
            questions: vec![] , 
            embedding_vector: None
        }
    }
}

impl HypeChunk {
    pub fn set_questions(mut self, questions: Vec<String>) -> Self {
        self.questions = questions;
        self
    }
}

impl Embeddable for HypeChunk {
    fn try_into_embed(&self) -> GenerateEmbeddingsRequest {
        GenerateEmbeddingsRequest::new(
            "bge-m3".to_owned(),
            EmbeddingsInput::Multiple(self.questions.clone())
        )
    }
    
    fn set_embedding_vectors(&mut self, embedding_vector: Vec<EmbeddingVector>) {
        self.embedding_vector = Some(embedding_vector);
    }
    
    fn prepare_for_upload(self, parent_doc: String, doc_summary: Option<String>) -> Result<Vec<EmbeddedChunk>> {
        let embedding_vectors = match self.embedding_vector {
            Some(v) => v,
            None => return Err(anyhow!("No embedding vectors on hype chunk")),
        };

        if self.questions.len() != embedding_vectors.len() {
            return Err(anyhow!("Number of questions and embeddings don't match on hypechunk"));
        }

        let questions_with_embeddings: Vec<(&String, EmbeddingVector)> = self
            .questions
            .iter()
            .zip(embedding_vectors.into_iter())
            .collect();

        let mut embedded_chunks = vec![];
        let doc_summary = if let Some(summ) = doc_summary {
            summ
        } else {
            "".to_string()
        };
        for (question, embedding_vector) in questions_with_embeddings.into_iter() {
            embedded_chunks.push(EmbeddedChunk {
                embedding_vector,
                id: uuid::Uuid::new_v4().to_string(),
                doc_id: parent_doc.clone(),
                doc_seq_num: self.seq_num,
                content: self.text.clone(),
                additional_data: Value::String(question.to_string()),
                doc_summary: doc_summary.clone()
            });
        }

        Ok(embedded_chunks)
    }

    
}
