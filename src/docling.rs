use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_VENV_DIR: &str = ".venv";
const DEFAULT_ARTIFACTS_DIR: &str = ".docling-models";
const DEFAULT_BIN: &str = ".local-bin/docling";

pub fn bootstrap() -> Result<()> {
    if env_bool("DUCLING_BOOTSTRAP", true) == false {
        return Ok(());
    }

    let venv_dir = env_path("DUCLING_VENV", DEFAULT_VENV_DIR);
    let artifacts_dir = env_path("DUCLING_ARTIFACTS_PATH", DEFAULT_ARTIFACTS_DIR);
    let bin_path = env_path("DUCLING_BIN", DEFAULT_BIN);

    ensure_venv(&venv_dir)?;
    ensure_docling_installed(&venv_dir)?;
    ensure_artifacts(&venv_dir, &artifacts_dir)?;
    ensure_wrapper(&venv_dir, &bin_path)?;

    Ok(())
}

pub fn bin_path() -> PathBuf {
    env_path("DUCLING_BIN", DEFAULT_BIN)
}

pub fn artifacts_path() -> PathBuf {
    env_path("DUCLING_ARTIFACTS_PATH", DEFAULT_ARTIFACTS_DIR)
}

pub fn num_threads() -> String {
    env::var("DUCLING_NUM_THREADS").unwrap_or_else(|_| "2".to_string())
}

pub fn device() -> String {
    env::var("DUCLING_DEVICE").unwrap_or_else(|_| "cpu".to_string())
}

pub fn ocr_enabled() -> bool {
    env_bool("DUCLING_OCR", false)
}

fn ensure_venv(venv_dir: &Path) -> Result<()> {
    let python = venv_dir.join("bin/python");
    if python.exists() {
        return Ok(());
    }

    println!("[DOCLING] creating Python virtualenv at {}", venv_dir.display());
    run_command(Command::new("python3").arg("-m").arg("venv").arg(venv_dir), "create Docling virtualenv")
}

fn ensure_docling_installed(venv_dir: &Path) -> Result<()> {
    let python = venv_dir.join("bin/python");
    let docling = venv_dir.join("bin/docling");

    if docling.exists() && !env_bool("DUCLING_UPDATE", false) {
        return Ok(());
    }

    println!("[DOCLING] installing/updating Python docling package");
    ensure_pip(&python)?;
    run_command(
        Command::new(&python).arg("-m").arg("pip").arg("install").arg("--upgrade").arg("pip"),
        "upgrade pip for Docling",
    )?;
    run_command(
        Command::new(&python).arg("-m").arg("pip").arg("install").arg("--upgrade").arg("docling"),
        "install Docling",
    )
}

fn ensure_pip(python: &Path) -> Result<()> {
    let has_pip = Command::new(python)
        .arg("-m")
        .arg("pip")
        .arg("--version")
        .output()
        .with_context(|| format!("check pip in {}", python.display()))?
        .status
        .success();

    if has_pip {
        return Ok(());
    }

    let ensurepip_result = run_command(
        Command::new(python).arg("-m").arg("ensurepip").arg("--upgrade"),
        "bootstrap pip for Docling",
    );
    if ensurepip_result.is_ok() {
        return Ok(());
    }

    println!("[DOCLING] ensurepip unavailable, bootstrapping pip with get-pip.py");
    run_command(
        Command::new(python).arg("-c").arg(
            r#"
import runpy
import tempfile
import urllib.request

url = "https://bootstrap.pypa.io/get-pip.py"
with tempfile.NamedTemporaryFile(suffix="-get-pip.py") as script:
    with urllib.request.urlopen(url, timeout=120) as response:
        script.write(response.read())
    script.flush()
    runpy.run_path(script.name, run_name="__main__")
"#,
        ),
        "bootstrap pip for Docling with get-pip.py",
    )
}

fn ensure_artifacts(venv_dir: &Path, artifacts_dir: &Path) -> Result<()> {
    if dir_has_entries(artifacts_dir)? && !env_bool("DUCLING_REFRESH_MODELS", false) {
        return Ok(());
    }

    let docling_tools = venv_dir.join("bin/docling-tools");
    println!("[DOCLING] downloading model artifacts to {}", artifacts_dir.display());
    fs::create_dir_all(artifacts_dir).with_context(|| format!("create Docling artifacts dir {}", artifacts_dir.display()))?;

    run_command(
        Command::new(docling_tools)
            .arg("models")
            .arg("download")
            .arg("-o")
            .arg(artifacts_dir)
            .arg("layout")
            .arg("tableformer")
            .arg("rapidocr"),
        "download Docling model artifacts",
    )
}

fn ensure_wrapper(venv_dir: &Path, bin_path: &Path) -> Result<()> {
    if let Some(parent) = bin_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create Docling wrapper dir {}", parent.display()))?;
    }

    let docling = path_for_script(&venv_dir.join("bin/docling"));
    let script = format!("#!/usr/bin/env bash\nexec \"{}\" \"$@\"\n", docling);
    let current = fs::read_to_string(bin_path).unwrap_or_default();
    if current != script {
        fs::write(bin_path, script).with_context(|| format!("write Docling wrapper {}", bin_path.display()))?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(bin_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(bin_path, perms)?;
    }

    Ok(())
}

fn run_command(command: &mut Command, action: &str) -> Result<()> {
    let output = command.output().with_context(|| format!("failed to run command to {action}"))?;

    if !output.status.success() {
        bail!(
            "failed to {action}: status {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn dir_has_entries(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let mut entries = fs::read_dir(path).with_context(|| format!("read Docling artifacts dir {}", path.display()))?;
    Ok(entries.next().transpose()?.is_some())
}

fn env_path(key: &str, default: &str) -> PathBuf {
    PathBuf::from(env::var(key).unwrap_or_else(|_| default.to_string()))
}

fn env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON"),
        Err(_) => default,
    }
}

fn path_for_script(path: &Path) -> String {
    path.to_string_lossy().replace('"', "\\\"")
}
