use std::{fs::File, io::{BufReader, Read}};
use anyhow::Result;
use crate::shared::file::WoodstockFileData;

use super::{loaded_data::LoadedFile, FileLoader};

pub struct MarkdownFileLoader;

impl FileLoader for MarkdownFileLoader {
    fn load_file(file: &WoodstockFileData) -> Result<LoadedFile> {
        let mut f = BufReader::new(File::open(&file.path)?);
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        Ok(LoadedFile {
            file_type: file.file_type.clone(),
            content: buffer,
            internal_id: file.internal_id,
            answers: file.answers.clone(),
            tags: file.tags.clone(),
        })
    }
}