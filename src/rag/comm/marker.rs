// src/services/marker.rs

use std::{collections::HashMap, env, path::Path};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone)]
pub struct MarkerClient {
    base_url: Url,
    http: reqwest::Client,
}

impl Default for MarkerClient {
    fn default() -> Self {
        let base = env::var("MARKER_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let base_url = Url::parse(&base).expect("Invalid MARKER_BASE_URL");
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed building reqwest client");
        Self { base_url, http }
    }
}

/// Options you can pass to Marker
#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    pub formats: Vec<Format>,           // default: ["markdown","json","chunks"]
    pub use_llm: bool,                  // default: false
    pub force_ocr: bool,                // default: false
    pub paginate_output: bool,          // default: false
    pub strip_existing_ocr: bool,       // default: false
    pub redo_inline_math: bool,         // default: false
}

#[derive(Debug, Clone)]
pub enum Format {
    Markdown,
    Json,
    Html,
    Chunks,
}

impl Format {
    fn as_str(&self) -> &'static str {
        match self {
            Format::Markdown => "markdown",
            Format::Json => "json",
            Format::Html => "html",
            Format::Chunks => "chunks",
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Markdown
    }
}

/// Concrete response types that mirror the server JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub job_id: String,
    pub outputs: Outputs,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Outputs {
    /// Full markdown text if requested
    pub markdown: Option<String>,
    /// Full HTML string if requested
    pub html: Option<String>,
    /// Structured JSON from Marker. Schema varies by document type, so keep flexible.
    pub json: Option<serde_json::Value>,
    /// Chunked text with basic fields commonly used in RAG
    pub chunks: Option<Vec<Chunk>>,
    /// Images as base64 mapped by filename
    pub images: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Chunk {
    pub text: String,
    #[serde(default)]
    pub page: Option<i32>,
    #[serde(default)]
    pub section_path: Option<Vec<String>>,
    /// carry any extra fields without failing deserialization
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    #[serde(default)]
    pub markdown_metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub json_metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub html_metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub chunks_metadata: Option<serde_json::Value>,
    /// carry any extra metadata keys
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl MarkerClient {
    /// Convenience helper for the common case: markdown, json, chunks
    pub async fn convert_file_common<P: AsRef<Path>>(&self, file_path: P) -> anyhow::Result<ConvertResponse> {
        let opts = ConvertOptions {
            formats: vec![Format::Markdown, Format::Json, Format::Chunks],
            ..Default::default()
        };
        self.convert_file_with_options(file_path, &opts).await
    }

    /// Full control over options
    pub async fn convert_file_with_options<P: AsRef<Path>>(
        &self,
        file_path: P,
        options: &ConvertOptions,
    ) -> anyhow::Result<ConvertResponse> {
        let url = self.base_url.join("convert")?;

        let formats_csv = if options.formats.is_empty() {
            "markdown,json,chunks".to_string()
        } else {
            options
                .formats
                .iter()
                .map(Format::as_str)
                .collect::<Vec<_>>()
                .join(",")
        };

        let file_part = multipart::Part::file(file_path.as_ref())
            .await?
            .file_name(
                file_path
                    .as_ref()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("input.bin")
                    .to_string(),
            )
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("formats", formats_csv)
            .text("use_llm", options.use_llm.to_string())
            .text("force_ocr", options.force_ocr.to_string())
            .text("paginate_output", options.paginate_output.to_string())
            .text("strip_existing_ocr", options.strip_existing_ocr.to_string())
            .text("redo_inline_math", options.redo_inline_math.to_string());

        let resp = self
            .http
            .post(url)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json::<ConvertResponse>()
            .await?;

        Ok(resp)
    }
}
