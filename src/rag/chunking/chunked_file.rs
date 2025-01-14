use crate::{rag::loading::loaded_data::LoadedFile, shared::{file::Answer, file_type::FileType}};

use super::chunk::Chunk;


#[derive(Debug)]
pub struct ChunkedFile {
    pub file_type: FileType,
    pub chunks: Vec<Chunk>,
    pub internal_id: i64,
    pub answers: Vec<Answer>,
    pub tags: Option<Vec<String>>,
}

impl From<(LoadedFile, Vec<Chunk>)> for ChunkedFile {
    fn from(value: (LoadedFile, Vec<Chunk>)) -> Self {
        let file = value.0;
        let chunks = value.1;
        Self { 
            file_type: file.file_type, 
            chunks, 
            internal_id: file.internal_id, 
            answers: file.answers, 
            tags: file.tags,
        }
    }
}