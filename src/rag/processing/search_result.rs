use ollama_rs::generation::completion::GenerationResponseStream;

use super::result_chunk::ResultChunk;



pub struct SearchResult {
    pub chunks: Vec<ResultChunk>,
    pub stream: GenerationResponseStream,
}