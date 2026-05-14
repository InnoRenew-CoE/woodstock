use crate::rag::{comm::marker::MarkerClient, RagProcessableFile};
use anyhow::{anyhow, Result};

use super::{loaded_data::LoadedFile, FileLoader, RagProcessableFileType};

pub struct PdfFileLoader;

impl FileLoader for PdfFileLoader {
    async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
        // let extracted_text = pdf_extract::extract_text(&file.path)
        //     .map_err(|err| anyhow!("Failed to extract text from PDF: {}", err))?;

        // if extracted_text.trim().is_empty() {
        //     println!(
        //         "Warning: No text could be extracted from '{}'. It may be an image-only PDF.",
        //         file.path.display()
        //     );
        // }

        let marker = MarkerClient::default();
        println!("Parsing PDF with Marker at {:#?}", file);
        let resp = marker.convert_file_common(&file.path)
            .await
            .map_err(|err| anyhow!("Marker conversion failed: {}", err))?;
        println!("PDF parsed: {:#?}", resp.job_id);
        let extracted_text = resp
            .outputs
            .markdown
            .ok_or_else(|| anyhow!("No markdown output from Marker"))?;

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