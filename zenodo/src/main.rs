use std::env;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;

mod db;

const MAX_FILE_SIZE: u64 = 30 * 1024 * 1024; // 30 MB

// ─── RagProcessableFile ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct RagProcessableFile {
    path: PathBuf,
    file_type: RagProcessableFileType,
    internal_id: String,
    original_name: String,
    file_description: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
enum RagProcessableFileType {
    PDF,
}

// ─── Zenodo API response types ──────────────────────────────────────────────

/// A published record from Zenodo.
#[derive(Debug, Deserialize)]
struct Record {
    id: u64,
    #[serde(default)]
    metadata: RecordMetadata,
    #[serde(default)]
    files: Vec<RecordFile>,
}

#[derive(Default, Debug, Deserialize)]
struct RecordMetadata {
    #[serde(default)]
    title: String,
}

/// A file attached to a record.
#[derive(Debug, Deserialize)]
struct RecordFile {
    key: String,
    #[serde(default)]
    links: RecordFileLinks,
}

#[derive(Default, Debug, Deserialize)]
struct RecordFileLinks {
    /// Zenodo returns `"self"` (a Rust keyword), so we rename it.
    #[serde(rename = "self", default)]
    self_: Option<String>,
    /// Direct download URL.
    #[serde(default)]
    content: Option<String>,
}

/// The records search API returns an envelope with pagination metadata.
#[derive(Debug, Deserialize)]
struct SearchResponse {
    hits: Hits,
}

#[derive(Debug, Deserialize)]
struct Hits {
    hits: Vec<Record>,
    total: u64,
}

// ─── Cursor tracking (page-based) ───────────────────────────────────────────

fn cursor_path() -> PathBuf {
    let path = env::var("ZENODO_CURSOR_PATH").unwrap_or_else(|_| ".zenodo_cursor".to_string());
    PathBuf::from(path)
}

/// Read the last-seen page number from the cursor file.
/// Returns 1 if no cursor exists yet (start at page 1).
async fn read_cursor() -> u64 {
    let path = cursor_path();
    fs::read_to_string(&path).await.ok().and_then(|s| s.trim().parse().ok()).unwrap_or(1)
}

/// Persist the next page number so we resume from here.
async fn write_cursor(page: u64) -> Result<()> {
    let path = cursor_path();
    fs::write(&path, page.to_string())
        .await
        .with_context(|| format!("failed to write cursor to {}", path.display()))
}

// ─── File download ──────────────────────────────────────────────────────────

async fn download_file(client: &Client, url: &str, dest_path: &PathBuf) -> Result<u64> {
    let response = client.get(url).send().await.with_context(|| format!("failed to GET {}", url))?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("download failed for {} with status {}", url, status);
    }

    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("failed to read response body for {}", url))?;

    fs::write(dest_path, &bytes)
        .await
        .with_context(|| format!("failed to write {}", dest_path.display()))?;

    Ok(bytes.len() as u64)
}

// ─── Main ───────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let _token = env::var("ZENODO_TOKEN").unwrap_or_default();
    let download_path = PathBuf::from(env::var("ZENODO_DOWNLOAD_PATH").context("ZENODO_DOWNLOAD_PATH not set")?);
    let query = env::var("ZENODO_QUERY").unwrap_or_default();

    let mut page = read_cursor().await;
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Zenodo downloader");
    println!("  Query:       {}", if query.is_empty() { "(none)" } else { &query });
    println!("  Start page:  {}", page);
    println!("  Download to: {}", download_path.display());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    fs::create_dir_all(&download_path)
        .await
        .with_context(|| format!("failed to create directory {}", download_path.display()))?;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("yaak")
        .build()
        .context("failed to build HTTP client")?;

    let db = db::build_db_client().await;

    let api_base = "https://zenodo.org/api/records";
    let mut downloaded = 0u64;

    'outer: loop {
        // Build query parameters
        let mut params: Vec<(String, String)> = Vec::new();
        if !query.is_empty() {
            params.push(("q".to_string(), query.clone()));
        }
        params.push(("page".to_string(), page.to_string()));
        params.push(("sort".to_string(), "mostrecent".to_string()));

        let url = reqwest::Url::parse_with_params(api_base, &params).context("failed to build request URL")?;

        println!("\n📡 Fetching page {} … {}", page, url);

        let response = client
            .get(url.clone())
            .header("Accept", "application/json")
            .send()
            .await
            .context("API request failed")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, body);
        }

        let search: SearchResponse = response.json().await.context("failed to parse API response as JSON")?;

        let hits = search.hits.hits;
        if hits.is_empty() {
            println!("  No more records found.");
            break;
        }

        for record in &hits {
            // if downloaded >= num_files {
            //     break 'outer;
            // }

            let title = &record.metadata.title;

            for file in &record.files {
                // Only download PDFs
                if !file.key.ends_with(".pdf") {
                    continue;
                }

                // Prefer the `content` link (direct download), fall back to `self`
                let download_url = file.links.content.as_deref().or(file.links.self_.as_deref());

                let url = match download_url {
                    Some(u) => u,
                    None => {
                        eprintln!("     ❌  No download URL for {}", file.key);
                        continue;
                    }
                };

                // Fetch file info with a HEAD request to check size before downloading
                let head_resp = client.head(url).send().await.context("HEAD request failed")?;

                let content_length = head_resp
                    .headers()
                    .get(reqwest::header::CONTENT_LENGTH)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());

                match content_length {
                    Some(size) if size > MAX_FILE_SIZE => {
                        println!("     ⏭  Skipping ({} exceeds 30 MB limit)", file.key);
                        continue;
                    }
                    Some(_) => {} // within limit, proceed
                    None => {
                        // No Content-Length header — proceed anyway
                    }
                }

                // Use a UUID for the folder and file name
                let uuid_str = Uuid::new_v4().to_string();
                let file_dir = download_path.join(&uuid_str);
                fs::create_dir_all(&file_dir)
                    .await
                    .with_context(|| format!("failed to create directory {}", file_dir.display()))?;

                let pdf_filename = format!("{}.pdf", uuid_str);
                let dest_path = file_dir.join(&pdf_filename);

                let id = db::insert_file(&db, &file.key, &pdf_filename, ".pdf", &1).await;
                if let Err(e) = id {
                    eprintln!("     ❌  Error inserting file: {:#}", e);
                    continue;
                }

                println!("     ⬇  Downloading: {} → {}", file.key, uuid_str);
                match download_file(&client, url, &dest_path).await {
                    Ok(bytes) => {
                        downloaded += 1;

                        // Verify the file was actually written
                        if !dest_path.exists() {
                            eprintln!("     ❌  File not found after download: {}", dest_path.display());
                            continue;
                        }

                        println!("     ✅  Saved: {} ({} bytes)", dest_path.display(), bytes);

                        // Write metadata.json
                        let metadata = RagProcessableFile {
                            path: dest_path.clone(),
                            file_type: RagProcessableFileType::PDF,
                            internal_id: uuid_str.clone(),
                            original_name: file.key.clone(),
                            file_description: None,
                            tags: None,
                        };
                        let meta_path = file_dir.join("metadata.json");
                        let meta_json = serde_json::to_string_pretty(&metadata)?;
                        fs::write(&meta_path, &meta_json)
                            .await
                            .with_context(|| format!("failed to write {}", meta_path.display()))?;

                        println!("     📝  Metadata written: {}", meta_path.display());
                    }
                    Err(e) => {
                        eprintln!("     ❌  Error downloading {}: {:#}", file.key, e);
                    }
                }
            }
        }
        page += 1;
    }

    // Persist cursor so next run picks up from the next unprocessed page
    write_cursor(page).await?;
    println!("\n📝 Cursor updated to page {}", page);

    println!("\n✅ Done — downloaded {} file(s).", downloaded);
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[.. max.saturating_sub(1)])
    }
}
