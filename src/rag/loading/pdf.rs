use crate::rag::RagProcessableFile;
use anyhow::{anyhow, Result};

use super::{loaded_data::LoadedFile, FileLoader, RagProcessableFileType};

pub struct PdfFileLoader;

impl FileLoader for PdfFileLoader {
    fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
        let extracted_text = pdf_extract::extract_text(&file.path)
            .map_err(|err| anyhow!("Failed to extract text from PDF: {}", err))?;

        if extracted_text.trim().is_empty() {
            println!(
                "Warning: No text could be extracted from '{}'. It may be an image-only PDF.",
                file.path.display()
            );
        }

        Ok(LoadedFile {
            file_type: RagProcessableFileType::Pdf,
            content: extracted_text,
            internal_id: file.internal_id.clone(),
            tags: file.tags.clone(),
            original_file_description: file.file_description.clone(),
            syntetic_file_description: None,
        })
    }
}