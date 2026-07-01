use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RagProcessableFileType {
    Text,
    Markdown,
    Pdf,
    Word,
    Presentation,
    Spreadsheet,
    Html,
    Epub,
    Audio,
    Subtitle,
    Email,
    Image,
    Latex,
    OpenDocument,
    Xbrl,
    DoclingDocument,
}

impl RagProcessableFileType {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.trim_start_matches('.').to_ascii_lowercase().as_str() {
            "txt" | "text" => Some(Self::Text),
            "md" | "markdown" | "qmd" | "rmd" => Some(Self::Markdown),
            "pdf" => Some(Self::Pdf),
            "docx" => Some(Self::Word),
            "pptx" => Some(Self::Presentation),
            "xlsx" | "xlsm" => Some(Self::Spreadsheet),
            "html" | "htm" => Some(Self::Html),
            "epub" => Some(Self::Epub),
            "wav" | "mp3" => Some(Self::Audio),
            "vtt" | "webvtt" => Some(Self::Subtitle),
            "eml" | "msg" => Some(Self::Email),
            "png" | "jpg" | "jpeg" | "tif" | "tiff" | "bmp" | "webp" => Some(Self::Image),
            "tex" | "latex" => Some(Self::Latex),
            "odt" | "ods" | "odp" => Some(Self::OpenDocument),
            "xbrl" | "xml" => Some(Self::Xbrl),
            "json" | "yaml" | "yml" => Some(Self::DoclingDocument),
            _ => None,
        }
    }

    pub fn supported_extensions() -> &'static [&'static str] {
        &[
            "txt", "text", "md", "markdown", "qmd", "rmd", "pdf", "docx", "pptx", "xlsx", "xlsm", "html", "htm", "epub", "wav", "mp3", "vtt",
            "webvtt", "eml", "msg", "png", "jpg", "jpeg", "tif", "tiff", "bmp", "webp", "tex", "latex", "odt", "ods", "odp", "xbrl", "xml", "json",
            "yaml", "yml",
        ]
    }

    pub fn is_docling_backed(&self) -> bool {
        !matches!(self, Self::Text | Self::Markdown)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagProcessableFile {
    pub path: PathBuf,
    pub file_type: RagProcessableFileType,
    pub internal_id: String,
    pub original_name: String,
    pub file_description: Option<String>,
    pub tags: Option<Vec<String>>,
}
