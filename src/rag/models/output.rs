use crate::rag::comm::ChatTextStream;

use crate::rag::models::chunks::ResultChunk;

pub struct SearchResult {
    pub chunks: Vec<ResultChunk>,
    pub stream: ChatTextStream,
}
