use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::docling;
use crate::rag::models::ImageRef;

static MARKDOWN_IMAGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"!\[(?P<alt>[^\]]*)\]\((?P<target>[^)\s]+)(?:\s+"[^"]*")?\)"#).unwrap());

pub struct DoclingConversion {
    pub markdown: String,
    pub images: Vec<ImageRef>,
}

pub struct DoclingClient {
    bin: PathBuf,
    artifacts_path: PathBuf,
    num_threads: String,
    device: String,
    ocr_enabled: bool,
}

impl Default for DoclingClient {
    fn default() -> Self {
        Self {
            bin: docling::bin_path(),
            artifacts_path: docling::artifacts_path(),
            num_threads: docling::num_threads(),
            device: docling::device(),
            ocr_enabled: docling::ocr_enabled(),
        }
    }
}

impl DoclingClient {
    pub fn convert_to_markdown_with_name<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        file_path: P,
        original_name: Option<S>,
        document_id: &str,
    ) -> Result<DoclingConversion> {
        let output_dir = std::env::temp_dir().join(format!("woodstock-docling-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&output_dir).with_context(|| format!("create Docling output dir {}", output_dir.display()))?;
        let input_path = prepare_input_path(file_path.as_ref(), original_name.as_ref().map(|name| name.as_ref()), &output_dir)?;

        let result = self
            .run_docling(&input_path, &output_dir)
            .and_then(|_| read_generated_markdown(&output_dir))
            .and_then(|(markdown, md_path)| persist_and_rewrite_images(&markdown, &md_path, document_id));

        fs::remove_dir_all(&output_dir).ok();
        result
    }

    fn run_docling(&self, file_path: &Path, output_dir: &Path) -> Result<()> {
        let mut command = Command::new(&self.bin);
        command
            .arg(file_path)
            .arg("--to")
            .arg("md")
            .arg("--image-export-mode")
            .arg("referenced")
            .arg("--output")
            .arg(output_dir)
            .arg("--num-threads")
            .arg(&self.num_threads)
            .arg("--device")
            .arg(&self.device)
            .arg("--artifacts-path")
            .arg(&self.artifacts_path);

        if !self.ocr_enabled {
            command.arg("--no-ocr");
        }

        let output = command
            .output()
            .with_context(|| format!("failed to run Docling CLI {}", self.bin.display()))?;

        if !output.status.success() {
            bail!(
                "Docling conversion failed: status {}\nstdout:\n{}\nstderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }
}

fn prepare_input_path(file_path: &Path, original_name: Option<&str>, output_dir: &Path) -> Result<PathBuf> {
    if file_path.extension().is_some() {
        return Ok(file_path.to_path_buf());
    }

    let Some(extension) = original_name
        .and_then(|name| Path::new(name).extension())
        .and_then(|extension| extension.to_str())
    else {
        return Ok(file_path.to_path_buf());
    };

    let input_path = output_dir.join(format!("input.{extension}"));
    fs::copy(file_path, &input_path).with_context(|| {
        format!(
            "copy extensionless staged file {} to Docling input {}",
            file_path.display(),
            input_path.display()
        )
    })?;
    Ok(input_path)
}

fn read_generated_markdown(output_dir: &Path) -> Result<(String, PathBuf)> {
    let md_path = find_first_markdown(output_dir)?.ok_or_else(|| anyhow!("Docling did not generate a markdown file in {}", output_dir.display()))?;
    let markdown = fs::read_to_string(&md_path).with_context(|| format!("read Docling markdown output {}", md_path.display()))?;
    Ok((markdown, md_path))
}

fn find_first_markdown(dir: &Path) -> Result<Option<PathBuf>> {
    let mut matches = Vec::new();
    collect_markdown_files(dir, &mut matches)?;
    matches.sort();
    Ok(matches.into_iter().next())
}

fn collect_markdown_files(dir: &Path, matches: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read Docling output dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_markdown_files(&path, matches)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        {
            matches.push(path);
        }
    }
    Ok(())
}

fn persist_and_rewrite_images(markdown: &str, md_path: &Path, document_id: &str) -> Result<DoclingConversion> {
    let image_root = docling::image_root().join(document_id);
    fs::create_dir_all(&image_root).with_context(|| format!("create RAG image dir {}", image_root.display()))?;

    let md_parent = md_path.parent().unwrap_or_else(|| Path::new("."));
    let mut images = Vec::new();
    let mut rewritten = String::with_capacity(markdown.len());
    let mut last = 0usize;

    for caps in MARKDOWN_IMAGE_RE.captures_iter(markdown) {
        let Some(full_match) = caps.get(0) else {
            continue;
        };
        let target = caps.name("target").map(|m| m.as_str()).unwrap_or_default();
        let alt = caps.name("alt").map(|m| m.as_str()).unwrap_or_default();

        if is_external_image_target(target) {
            continue;
        }

        let source_path = md_parent.join(target);
        if !source_path.is_file() {
            continue;
        }

        let extension = source_path.extension().and_then(|ext| ext.to_str()).unwrap_or("bin");
        let image_id = uuid::Uuid::new_v4().to_string();
        let file_name = format!("{image_id}.{extension}");
        let target_path = image_root.join(&file_name);
        fs::copy(&source_path, &target_path)
            .with_context(|| format!("copy Docling image {} to {}", source_path.display(), target_path.display()))?;

        let route = format!("/rag/images/{document_id}/{file_name}");
        images.push(ImageRef {
            id: image_id,
            document_id: document_id.to_string(),
            file_name,
            route: route.clone(),
            alt_text: if alt.is_empty() { None } else { Some(alt.to_string()) },
            path: target_path,
        });

        rewritten.push_str(&markdown[last..full_match.start()]);
        rewritten.push_str(&format!("![{alt}]({route})"));
        last = full_match.end();
    }

    rewritten.push_str(&markdown[last..]);

    Ok(DoclingConversion { markdown: rewritten, images })
}

fn is_external_image_target(target: &str) -> bool {
    target.starts_with("http://") || target.starts_with("https://") || target.starts_with("data:") || target.starts_with('/')
}
