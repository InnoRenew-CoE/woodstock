use anyhow::Result;

use crate::shared::{file::WoodstockFileData, file_type::FileType};

use super::{loaded_data::LoadedFile, markdown::MarkdownFileLoader, pdf::PdfFileLoader, text::TextFileLoader};

pub trait FileLoader {
    fn load_file(file: &WoodstockFileData) -> Result<LoadedFile>;
}

pub fn load_file(file: &WoodstockFileData) -> Result<LoadedFile> {
    match file.file_type {
        FileType::Text => TextFileLoader::load_file(file),
        FileType::Markdown => MarkdownFileLoader::load_file(file),
        FileType::Pdf => PdfFileLoader::load_file(file),
    }
}

