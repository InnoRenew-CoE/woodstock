use std::collections::HashSet;

use qdrant_client::qdrant::SearchResponse;

use crate::rag::models::chunks::ResultChunk;

pub fn dedup(search: SearchResponse) -> Vec<ResultChunk> {
    let mut result_chunks: Vec<ResultChunk> = search
        .result
        .into_iter()
        .map(|r| r.into())
        .collect();


    let mut seen = HashSet::new();
    result_chunks.retain(|chunk| {
        seen.insert((chunk.doc_id.clone(), chunk.doc_seq_num))
    });

    result_chunks
}