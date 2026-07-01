use std::env;
use std::path::PathBuf;

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::fs;

pub fn staging_dir() -> PathBuf {
    PathBuf::from(env::var("STAGING_FOLDER").unwrap_or_else(|_| "./staging".to_string()))
}

fn submission_dir(stage: &str, uuid: &str) -> PathBuf {
    staging_dir().join(stage).join(uuid)
}

pub fn metadata_path(stage: &str, uuid: &str) -> PathBuf {
    submission_dir(stage, uuid).join("metadata.json")
}

pub fn answers_path(stage: &str, uuid: &str) -> PathBuf {
    submission_dir(stage, uuid).join("answers.json")
}

pub fn loaded_path(stage: &str, uuid: &str) -> PathBuf {
    submission_dir(stage, uuid).join("loaded.json")
}

pub fn chunked_path(stage: &str, uuid: &str) -> PathBuf {
    submission_dir(stage, uuid).join("chunked.json")
}

pub fn hyped_path(stage: &str, uuid: &str) -> PathBuf {
    submission_dir(stage, uuid).join("hyped.json")
}

pub async fn write_json<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    let bytes = serde_json::to_vec(data)?;
    fs::write(path, bytes).await?;
    Ok(())
}

pub async fn read_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let bytes = fs::read(path).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub async fn move_submission(from: &PathBuf, to: &PathBuf) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::rename(from, to).await?;
    Ok(())
}

pub async fn move_to_failed(folder: &PathBuf, stage: &str) -> Result<()> {
    let uuid = folder.file_name().unwrap().to_string_lossy().to_string();
    let failed_dir = staging_dir().join(stage).join("failed").join(&uuid);
    move_submission(folder, &failed_dir).await
}
