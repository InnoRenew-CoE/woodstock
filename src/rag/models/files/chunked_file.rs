use crate::{rag::{comm::embedding::Embeddable, loading::loaded_data::LoadedFile, models::RagProcessableFileType}, shared::{file::Answer, file_type::FileType}};

#[derive(Debug)]
pub struct ChunkedFile<T> where T: Embeddable {
    pub file_type: RagProcessableFileType,
    pub chunks: Vec<T>,
    pub internal_id: String,
    pub original_file_description: Option<String>,
    pub syntetic_file_description: Option<String>,
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
            tags: file.tags,
            original_file_description: file.original_file_description,
            syntetic_file_description: file.syntetic_file_description,
            
        }
    }
}