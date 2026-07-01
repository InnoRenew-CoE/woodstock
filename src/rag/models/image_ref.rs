use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRef {
    pub id: String,
    pub document_id: String,
    pub file_name: String,
    pub route: String,
    pub alt_text: Option<String>,
    pub path: PathBuf,
}
