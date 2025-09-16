// src/services/marker.rs

use std::{collections::HashMap, env, path::Path, time::Duration};
use serde::{Deserialize, Serialize};
use url::Url;
use std::{fs};
use reqwest::{Client, multipart};

#[derive(Clone)]
pub struct MarkerClient {
    base_url: Url,
    http: Client,
    admin_token: String,
}


impl Default for MarkerClient {
    fn default() -> Self {
        let base_url = env::var("MARKER_BASE_URL")
            .expect("MARKER_BASE_URL not set");
        let base_url = Url::parse(&base_url)
            .expect("Invalid MARKER_BASE_URL");
        let admin_token = env::var("ADMIN_TOKEN")
            .expect("Marker ADMIN_TOKEN not set");

        let http = Client::builder()
            .no_proxy()   
            .http1_only() 
            .connect_timeout(std::time::Duration::from_secs(3))
            .build()
            .expect("reqwest client");

        Self { base_url, http, admin_token }
    }
}

/// Options you can pass to Marker
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    pub formats: Vec<Format>,     
    pub use_llm: bool,            
    pub force_ocr: bool,          
    pub paginate_output: bool,    
    pub strip_existing_ocr: bool, 
    pub redo_inline_math: bool,   
    pub return_images: bool,      
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self {
            formats: vec![Format::Markdown],
            use_llm: false,
            force_ocr: false,
            paginate_output: false,
            strip_existing_ocr: false,
            redo_inline_math: false,
            return_images: false,
        }
    }
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
    pub markdown: Option<String>,
    pub html: Option<String>,
    pub json: Option<serde_json::Value>,
    pub chunks: Option<Vec<Chunk>>,
    pub images: Option<HashMap<String, String>>, // base64
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Chunk {
    pub text: String,
    #[serde(default)]
    pub page: Option<i32>,
    #[serde(default)]
    pub section_path: Option<Vec<String>>,
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
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl MarkerClient {

    /// Convenience helper for the common case
    pub async  fn convert_file_common<P: AsRef<Path>>(&self, file_path: P) -> anyhow::Result<ConvertResponse> {
        let opts = ConvertOptions::default();
        self.convert_file(file_path, &opts).await
    }

    pub async fn convert_file<P: AsRef<Path>>(
        &self,
        file_path: P,
        options: &ConvertOptions,
    ) -> anyhow::Result<ConvertResponse> {
        let base_url = self.base_url.clone();
        let path = file_path.as_ref().to_path_buf();
        
        let url = base_url.join("convert")?;

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

        let data = fs::read(&path)?;
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str()).unwrap_or("input.bin")
            .to_string();
        let file_part = multipart::Part::bytes(data).file_name(file_name);

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("formats", formats_csv)
            .text("use_llm", options.use_llm.to_string())
            .text("force_ocr", options.force_ocr.to_string())
            .text("paginate_output", options.paginate_output.to_string())
            .text("strip_existing_ocr", options.strip_existing_ocr.to_string())
            .text("redo_inline_math", options.redo_inline_math.to_string())
            .text("return_images", options.return_images.to_string());

        let resp = self.http
            .post(url.clone())
            .bearer_auth(&self.admin_token)
            .multipart(form)
            .timeout(Duration::from_secs(28800)) 
            .send()
            .await?;

        let status = resp.status();
        if resp.status().is_client_error() || resp.status().is_server_error() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("marker {}: {}", status, body);
        }

        let out = resp.json::<ConvertResponse>().await?;
        Ok(out)
    }

}

