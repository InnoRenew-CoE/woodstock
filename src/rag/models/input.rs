use std::path::PathBuf;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagProcessableFile {
    pub path: PathBuf,
    pub internal_id: String,
    pub original_name: String,
    pub file_description: String,
    pub tags: Option<Vec<String>>,
}

