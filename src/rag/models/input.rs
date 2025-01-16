use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RagProcessableFileType {
    Text,
    Markdown,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagProcessableFile {
    pub path: PathBuf,
    pub file_type: RagProcessableFileType,
    pub internal_id: String,
    pub original_name: String,
    pub file_description: Option<String>,
    pub tags: Option<Vec<String>>,
}

