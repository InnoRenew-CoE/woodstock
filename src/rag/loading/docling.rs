use crate::rag::{comm::docling::DoclingClient, RagProcessableFile};
use anyhow::{anyhow, Result};

use super::{loaded_data::LoadedFile, FileLoader};

pub struct DoclingFileLoader;

impl FileLoader for DoclingFileLoader {
    async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
        let docling = DoclingClient::default();
        println!("Parsing document with Docling at {:#?}", file);
        let path = file.path.clone();
        let original_name = file.original_name.clone();
        let document_id = file.internal_id.clone();
        let converted = tokio::task::spawn_blocking(move || docling.convert_to_markdown_with_name(path, Some(original_name), &document_id))
            .await
            .map_err(|err| anyhow!("Docling conversion task failed: {}", err))?
            .map_err(|err| anyhow!("Docling conversion failed: {}", err))?;
        println!("Document parsed with Docling");

        Ok(LoadedFile {
            file_type: file.file_type.clone(),
            content: converted.markdown,
            internal_id: file.internal_id.clone(),
            tags: file.tags.clone(),
            original_file_description: file.file_description.clone(),
            syntetic_file_description: None,
            images: converted.images,
        })
    }
}
