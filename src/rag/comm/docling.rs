use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::docling;

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
    pub fn convert_to_markdown_with_name<P: AsRef<Path>, S: AsRef<str>>(&self, file_path: P, original_name: Option<S>) -> Result<String> {
        let output_dir = std::env::temp_dir().join(format!("woodstock-docling-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&output_dir).with_context(|| format!("create Docling output dir {}", output_dir.display()))?;
        let input_path = prepare_input_path(file_path.as_ref(), original_name.as_ref().map(|name| name.as_ref()), &output_dir)?;

        let result = self
            .run_docling(&input_path, &output_dir)
            .and_then(|_| read_generated_markdown(&output_dir));

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

fn read_generated_markdown(output_dir: &Path) -> Result<String> {
    let md_path = find_first_markdown(output_dir)?.ok_or_else(|| anyhow!("Docling did not generate a markdown file in {}", output_dir.display()))?;
    fs::read_to_string(&md_path).with_context(|| format!("read Docling markdown output {}", md_path.display()))
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
