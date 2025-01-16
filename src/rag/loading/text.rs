use std::{fs::File, io::{BufReader, Read}};
use anyhow::Result;
use crate::rag::RagProcessableFile;

use super::{loaded_data::LoadedFile, FileLoader, RagProcessableFileType};

pub struct TextFileLoader;

impl FileLoader for TextFileLoader {
    fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
        let mut f = BufReader::new(File::open(&file.path)?);
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        Ok(LoadedFile {
            file_type: RagProcessableFileType::Text,
            content: buffer,
            internal_id: file.internal_id.clone(),
            tags: file.tags.clone(),
            original_file_description: file.file_description.clone(),
            syntetic_file_description: None,
            
        })
    }
}