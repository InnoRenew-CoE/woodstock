use crate::shared::file::WoodstockFileData;
use anyhow::{Result, anyhow};
use lopdf::Document;

use super::{loaded_data::LoadedFile, FileLoader};

pub struct PdfFileLoader;

impl FileLoader for PdfFileLoader {
    fn load_file(file: &WoodstockFileData) -> Result<LoadedFile> {
        let doc = Document::load(&file.path)
            .map_err(|err| anyhow!(err.to_string()))?;

        let pages = doc.get_pages();
        let mut extracted_text = String::new();

        for (page_num, _) in pages {
            if doc.is_encrypted() {
                println!("ENCRIP");
            }
            
            let page_text = doc
                .extract_text(&vec![page_num])?
                .replace("?Identity-H Unimplemented?", "");

            extracted_text.push_str(&page_text);
        }

        Ok(LoadedFile {
            file_type: file.file_type.clone(),
            content: extracted_text,
            internal_id: file.internal_id,
            answers: file.answers.clone(),
            tags: file.tags.clone(),
        })
    }

}