use crate::rag::RagProcessableFileType;

#[derive(Debug)]
pub struct LoadedFile {
    pub file_type: RagProcessableFileType,
    pub content: String,
    pub internal_id: String,
    pub tags: Option<Vec<String>>,
}