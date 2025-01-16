use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RagProcessableFileType {
    Text,
    Markdown,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagProcessableFile {
    pub path: String,
    pub internal_id: String,
    pub original_name: String,
    pub tags: Option<Vec<String>>,
    pub file_type: RagProcessableFileType,
}

