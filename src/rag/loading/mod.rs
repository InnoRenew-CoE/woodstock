use loaded_data::LoadedFile;
use anyhow::Result;
use markdown::MarkdownFileLoader;
use pdf::PdfFileLoader;
use text::TextFileLoader;
use crate::shared::{file::WoodstockFileData, file_type::FileType};

pub mod loaded_data;
mod text;
mod markdown;
mod pdf;

trait FileLoader {
    fn load_file(file: &WoodstockFileData) -> Result<LoadedFile>;
}

pub fn load_file(file: &WoodstockFileData) -> Result<LoadedFile> {
    match file.file_type {
        FileType::Text => TextFileLoader::load_file(file),
        FileType::Markdown => MarkdownFileLoader::load_file(file),
        FileType::Pdf => PdfFileLoader::load_file(file),
    }
}

