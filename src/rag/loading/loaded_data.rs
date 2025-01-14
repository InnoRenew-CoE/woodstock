use crate::shared::{file::Answer, file_type::FileType};

#[derive(Debug)]
pub struct LoadedFile {
    pub file_type: FileType,
    pub content: String,
    pub internal_id: i64,
    pub answers: Vec<Answer>,
    pub tags: Option<Vec<String>>,
}

