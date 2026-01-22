use anyhow::Result;
use loaded_data::LoadedFile;
use markdown::MarkdownFileLoader;
use pdf::PdfFileLoader;
use text::TextFileLoader;

use super::{models::RagProcessableFileType, RagProcessableFile};

pub mod loaded_data;
mod markdown;
mod pdf;
mod text;

trait FileLoader {
    async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile>;
}

pub async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
    match file.file_type {
        RagProcessableFileType::Text => TextFileLoader::load_file(file).await,
        RagProcessableFileType::Markdown => MarkdownFileLoader::load_file(file).await,
        RagProcessableFileType::Pdf => PdfFileLoader::load_file(file).await,
    }
}
