use std::env;
use ollama_rs::{
    error::OllamaError, 
    generation::{
        completion::GenerationResponse, 
        embeddings::{request::GenerateEmbeddingsRequest, GenerateEmbeddingsResponse}
    }, 
    Ollama
};
use question::Questiton;

pub mod embedding;
mod question;

#[derive(Debug)]
pub struct OllamaClient {
    ollama: Ollama,
}

impl Default for OllamaClient {
    fn default() -> Self {
        let ollama_host = env::var("OLLAMA_HOST").expect("OLLAMA HOST not set");
        let ollama_port = env::var("OLLAMA_PORT").expect("OLLAMA PORT not set");
        let ollama_port: u16 = ollama_port.parse().expect("OLLAMA_PORT not u16");

        Self { 
            ollama: Ollama::new(ollama_host, ollama_port) 
        }
    }
}

impl OllamaClient {
    pub async fn generate(&self, question: Questiton) -> Result<GenerationResponse, OllamaError> {
        self.ollama.generate(question.into()).await
    }

    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<GenerateEmbeddingsResponse, OllamaError> {
        let request = GenerateEmbeddingsRequest::new("llama2:latest".to_string(), vec!["Why is the sky blue?", "Why is the sky red?"].into());
        self.ollama.generate_embeddings(request).await
    }
}
 