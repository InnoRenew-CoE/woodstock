use ollama_rs::generation::completion::GenerationResponseStream;

use crate::rag::models::chunks::ResultChunk;


pub struct SearchResult {
    pub chunks: Vec<ResultChunk>,
    pub stream: GenerationResponseStream,
}