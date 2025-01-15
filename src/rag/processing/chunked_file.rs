use crate::{rag::{comm::embedding::Embeddable, loading::loaded_data::LoadedFile}, shared::{file::Answer, file_type::FileType}};

#[derive(Debug)]
pub struct ChunkedFile<T> where T: Embeddable {
    pub file_type: FileType,
    pub chunks: Vec<T>,
    pub internal_id: i64,
    pub answers: Vec<Answer>,
    pub tags: Option<Vec<String>>,
}

impl<T> From<(LoadedFile, Vec<T>)> for ChunkedFile<T>
where
    T: Embeddable,
{
    fn from(value: (LoadedFile, Vec<T>)) -> Self {
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