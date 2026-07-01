use anyhow::Result;
use docling::DoclingFileLoader;
use loaded_data::LoadedFile;
use markdown::MarkdownFileLoader;
use text::TextFileLoader;

use super::{models::RagProcessableFileType, RagProcessableFile};

mod docling;
pub mod loaded_data;
mod markdown;
mod text;

trait FileLoader {
    async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile>;
}

pub async fn load_file(file: &RagProcessableFile) -> Result<LoadedFile> {
    match file.file_type {
        RagProcessableFileType::Text => TextFileLoader::load_file(file).await,
        RagProcessableFileType::Markdown => MarkdownFileLoader::load_file(file).await,
        _ if file.file_type.is_docling_backed() => DoclingFileLoader::load_file(file).await,
        _ => unreachable!("all file types are covered by direct or Docling loaders"),
    }
}
