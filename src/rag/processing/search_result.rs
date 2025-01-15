use super::result_chunk::ResultChunk;



pub struct SearchResult {
    chunks: Vec<ResultChunk>,
    doc_id: Vec<i32>,
}