use qdrant_client::qdrant::PointStruct;
use serde_json::{Map, Value};

use crate::rag::comm::embedding::EmbeddingVector;


#[derive(Debug)]
pub struct EmbeddedChunk {
    pub embedding_vector: EmbeddingVector,
    pub id: String,
    pub doc_id: String,
    pub doc_seq_num: i32,
    pub doc_summary: String,
    pub content: String,
    pub additional_data: Value,
}

impl Into<PointStruct> for EmbeddedChunk {
    fn into(self) -> PointStruct {
        let mut payload = Map::new();
        payload.insert("doc_id".to_string(), Value::String(self.doc_id));
        payload.insert("doc_seq_num".to_string(), Value::Number(self.doc_seq_num.into()));
        payload.insert("doc_summary".to_string(), Value::String(self.doc_summary));
        payload.insert("content".to_string(), Value::String(self.content));
        payload.insert("additional_data".to_string(), self.additional_data);

        PointStruct::new(
            self.id,
            self.embedding_vector.0,
            payload,
        )
    }
}
