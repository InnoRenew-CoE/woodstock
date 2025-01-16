use crate::rag::models::RagProcessableFileType;

#[derive(Debug)]
pub struct LoadedFile {
    pub file_type: RagProcessableFileType,
    pub content: String,
    pub original_file_description: Option<String>,
    pub syntetic_file_description: Option<String>,
    pub internal_id: String,
    pub tags: Option<Vec<String>>,
}