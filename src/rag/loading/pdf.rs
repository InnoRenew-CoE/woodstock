use crate::shared::file::WoodstockFileData;
use anyhow::{Result, anyhow};
use lopdf::{content::Operation, Document};

use super::{loaded_data::LoadedFile, loader::FileLoader};

pub struct PdfFileLoader;
impl FileLoader for PdfFileLoader {
    fn load_file(file: &WoodstockFileData) -> Result<LoadedFile> {
        // 1. Parse the PDF document.
        let doc = Document::load(&file.path)
            .map_err(|err| anyhow!(err.to_string()))?;

        // 2. Get a mapping of page_number -> page object ID.
        let pages = doc.get_pages();

        // 3. We'll accumulate text from all pages here.
        let mut extracted_text = String::new();

        for (page_num, _) in pages {
            if doc.is_encrypted() {
                println!("ENCRIP");
            }
            
            let page_text = doc
                .extract_text(&vec![page_num])?
                .replace("?Identity-H Unimplemented?", "");
        

            // e) Add page text + a page-break or newline
            extracted_text.push_str(&format!(
                "\n-- Page {} --\n{}",
                page_num, page_text
            ));
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