use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::{RetrievedPoint, ScoredPoint};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResultChunk {
    pub id: String,
    pub doc_id: String,
    pub doc_seq_num: i32,
    pub content: String,
    pub additional_data: Value,
    pub score: f32,
}

impl ResultChunk {
    pub fn short_id(&self) -> &str {
        &self.id
    }
}

impl From<ScoredPoint> for ResultChunk {
    fn from(value: ScoredPoint) -> Self {
        let id = id_from_point(value.id);
        Self::from_payload(id, value.payload, value.score)
    }
}

impl From<RetrievedPoint> for ResultChunk {
    fn from(value: RetrievedPoint) -> Self {
        let id = id_from_point(value.id);
        Self::from_payload(id, value.payload, 1.0)
    }
}

fn id_from_point(id: Option<qdrant_client::qdrant::PointId>) -> String {
    match id {
            Some(d) => match d.point_id_options {
                Some(PointIdOptions::Uuid(uuid)) => uuid,
                _ => format!("{:?}", d),
            },
            None => "Unknown".into(),
    }
}

impl ResultChunk {
    fn from_payload(id: String, payload: std::collections::HashMap<String, qdrant_client::qdrant::Value>, score: f32) -> Self {
        let doc_id = match payload.get("doc_id") {
            Some(d) => d.as_str().map_or("Unknown", |v| v),
            None => "Unknown",
        };
        let doc_id = doc_id.to_string();

        let doc_seq_num = match payload.get("doc_seq_num") {
            Some(d) => d.as_integer().unwrap_or(-1) as i32,
            None => -1,
        };

        let content: String = match payload.get("content") {
            Some(d) => d.as_str().map_or("".into(), |v| v.into()),
            None => "".into(),
        };

        let additional_data = match payload.get("additional_data") {
            Some(d) => d.to_owned(),
            None => Value::Null.into(),
        };

        Self {
            id,
            doc_id,
            doc_seq_num,
            content,
            additional_data: additional_data.into(),
            score,
        }
    }
}

impl Into<String> for &ResultChunk {
    fn into(self) -> String {
        format!(r#"
            ---
            Document containing this passage: {}
            Metadata: {:#?}

            Passage content: 
            
            {}
            
            ---

            "#,
            self.doc_id,
            self.additional_data,
            self.content
        )
    }
}
