use loaded_data::LoadedFile;
use anyhow::{Result, anyhow};
use markdown::MarkdownFileLoader;
use pdf::PdfFileLoader;
use text::TextFileLoader;

use super::RagProcessableFile;

pub mod loaded_data;
mod text;
mod markdown;
mod pdf;

#[derive(Debug, Clone)]
pub enum RagProcessableFileType {
    Text,
    Markdown,
    Pdf,
}


trait FileLoader {
    fn load_file(file: &RagProcessableFile) -> Result<LoadedFile>;
}

pub fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
    let extension = file
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    let file_type = match extension.as_str() {
        "pdf" => RagProcessableFileType::Pdf,
        "md"  => RagProcessableFileType::Markdown,
        "txt" => RagProcessableFileType::Text,
        _ => {
            return Err(anyhow!("Unsupported file type."));
        }
    };
    match file_type {
        RagProcessableFileType::Text => TextFileLoader::load_file(file),
        RagProcessableFileType::Markdown => MarkdownFileLoader::load_file(file),
        RagProcessableFileType::Pdf => PdfFileLoader::load_file(file),
    }
}

