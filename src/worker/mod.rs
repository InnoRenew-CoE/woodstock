use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use tokio::fs;

pub mod stage_io;
use stage_io::*;

use crate::db;
use crate::rag::Rag;

pub async fn run(rag: Rag) -> Result<()> {
    println!("[WORKER] Starting background worker");
    loop {
        if let Err(e) = process_new(&rag).await {
            eprintln!("[WORKER] process_new error: {e}");
        }
        if let Err(e) = process_loaded(&rag).await {
            eprintln!("[WORKER] process_loaded error: {e}");
        }
        if let Err(e) = process_chunked(&rag).await {
            eprintln!("[WORKER] process_chunked error: {e}");
        }
        if let Err(e) = process_hyped(&rag).await {
            eprintln!("[WORKER] process_hyped error: {e}");
        }
        if let Err(e) = process_embedded(&rag).await {
            eprintln!("[WORKER] process_embedded error: {e}");
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn list_submissions(stage: &str) -> Result<Vec<String>> {
    let dir = staging_dir().join(stage);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut read = fs::read_dir(&dir).await?;
    let mut uuids = Vec::new();
    while let Some(entry) = read.next_entry().await? {
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "failed" {
            continue;
        }
        if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
            uuids.push(name);
        }
    }
    uuids.sort();
    Ok(uuids)
}

async fn process_new(rag: &Rag) -> Result<()> {
    for uuid in list_submissions("new").await? {
        let folder = staging_dir().join("new").join(&uuid);
        let metadata_path = metadata_path("new", &uuid);
        let Ok(metadata) = read_json::<crate::rag::RagProcessableFile>(&metadata_path).await else {
            eprintln!("[WORKER] {uuid}: missing metadata.json");
            move_to_failed(&folder, "new").await.ok();
            continue;
        };
        match rag.insert_meta(&metadata).await {
            Ok(loaded) => {
                if let Err(e) = write_json(&loaded_path("new", &uuid), &loaded).await {
                    eprintln!("[WORKER] {uuid}: failed to write loaded.json: {e}");
                    move_to_failed(&folder, "new").await.ok();
                    continue;
                }
                let to = staging_dir().join("loaded").join(&uuid);
                if let Err(e) = move_submission(&folder, &to).await {
                    eprintln!("[WORKER] {uuid}: failed to move to loaded: {e}");
                }
                println!("[WORKER] {uuid}: loaded -> loaded");
            }
            Err(e) => {
                eprintln!("[WORKER] {uuid}: load failed: {e}");
                move_to_failed(&folder, "new").await.ok();
            }
        }
    }
    Ok(())
}

async fn process_loaded(_rag: &Rag) -> Result<()> {
    for uuid in list_submissions("loaded").await? {
        let folder = staging_dir().join("loaded").join(&uuid);
        let loaded_path = loaded_path("loaded", &uuid);
        let Ok(loaded) = read_json::<crate::rag::LoadedFile>(&loaded_path).await else {
            eprintln!("[WORKER] {uuid}: missing loaded.json");
            move_to_failed(&folder, "loaded").await.ok();
            continue;
        };
        let chunked = Rag::insert_chunk(loaded);
        if let Err(e) = write_json(&chunked_path("loaded", &uuid), &chunked).await {
            eprintln!("[WORKER] {uuid}: failed to write chunked.json: {e}");
            move_to_failed(&folder, "loaded").await.ok();
            continue;
        }
        let to = staging_dir().join("chunked").join(&uuid);
        if let Err(e) = move_submission(&folder, &to).await {
            eprintln!("[WORKER] {uuid}: failed to move to chunked: {e}");
        }
        println!("[WORKER] {uuid}: chunked -> chunked");
    }
    Ok(())
}

async fn process_chunked(rag: &Rag) -> Result<()> {
    for uuid in list_submissions("chunked").await? {
        let folder = staging_dir().join("chunked").join(&uuid);
        let chunked_path = chunked_path("chunked", &uuid);
        let Ok(chunked) = read_json::<crate::rag::ChunkedFile<crate::rag::Chunk>>(&chunked_path).await else {
            eprintln!("[WORKER] {uuid}: missing chunked.json");
            move_to_failed(&folder, "chunked").await.ok();
            continue;
        };
        match rag.insert_hype(chunked).await {
            Ok(hyped) => {
                if let Err(e) = write_json(&hyped_path("chunked", &uuid), &hyped).await {
                    eprintln!("[WORKER] {uuid}: failed to write hyped.json: {e}");
                    move_to_failed(&folder, "chunked").await.ok();
                    continue;
                }
                let to = staging_dir().join("hyped").join(&uuid);
                if let Err(e) = move_submission(&folder, &to).await {
                    eprintln!("[WORKER] {uuid}: failed to move to hyped: {e}");
                }
                println!("[WORKER] {uuid}: hyped -> hyped");
            }
            Err(e) => {
                eprintln!("[WORKER] {uuid}: hype failed: {e}");
                move_to_failed(&folder, "chunked").await.ok();
            }
        }
    }
    Ok(())
}

async fn process_hyped(rag: &Rag) -> Result<()> {
    for uuid in list_submissions("hyped").await? {
        let folder = staging_dir().join("hyped").join(&uuid);
        let hyped_path = hyped_path("hyped", &uuid);
        let Ok(hyped) = read_json::<crate::rag::ChunkedFile<crate::rag::HypeChunk>>(&hyped_path).await else {
            eprintln!("[WORKER] {uuid}: missing hyped.json");
            move_to_failed(&folder, "hyped").await.ok();
            continue;
        };
        match rag.insert_embed(hyped).await {
            Ok(embedded_chunks) => {
                if let Err(e) = write_json(
                    &staging_dir().join("hyped").join(&uuid).join("embedded.json"),
                    &embedded_chunks,
                ).await {
                    eprintln!("[WORKER] {uuid}: failed to write embedded.json: {e}");
                    move_to_failed(&folder, "hyped").await.ok();
                    continue;
                }
                let to = staging_dir().join("embedded").join(&uuid);
                if let Err(e) = move_submission(&folder, &to).await {
                    eprintln!("[WORKER] {uuid}: failed to move to embedded: {e}");
                }
                println!("[WORKER] {uuid}: embedded -> embedded");
            }
            Err(e) => {
                eprintln!("[WORKER] {uuid}: embed failed: {e}");
                move_to_failed(&folder, "hyped").await.ok();
            }
        }
    }
    Ok(())
}

async fn process_embedded(_rag: &Rag) -> Result<()> {
    for uuid in list_submissions("embedded").await? {
        let folder = staging_dir().join("embedded").join(&uuid);
        let embedded_path = staging_dir().join("embedded").join(&uuid).join("embedded.json");
        let Ok(embedded_chunks) = read_json::<Vec<crate::rag::EmbeddedChunk>>(&embedded_path).await else {
            eprintln!("[WORKER] {uuid}: missing embedded.json");
            move_to_failed(&folder, "embedded").await.ok();
            continue;
        };
        match Rag::insert_qdrant(embedded_chunks).await {
            Ok(()) => {
                // update DB status to done
                let metadata_path = metadata_path("embedded", &uuid);
                if let Ok(meta) = read_json::<crate::rag::RagProcessableFile>(&metadata_path).await {
                    let client = db::build_db_client().await;
                    let _ = db::update_file_status(&client, &meta.internal_id, "done", None).await;
                }
                // remove staging folder (original file stays at FILES_FOLDER/{uuid})
                fs::remove_dir_all(&folder).await.ok();
                println!("[WORKER] {uuid}: done -> qdrant upserted");
            }
            Err(e) => {
                eprintln!("[WORKER] {uuid}: qdrant upsert failed: {e}");
                let metadata_path = metadata_path("embedded", &uuid);
                if let Ok(meta) = read_json::<crate::rag::RagProcessableFile>(&metadata_path).await {
                    let client = db::build_db_client().await;
                    let _ = db::update_file_status(&client, &meta.internal_id, "failed", Some(&e.to_string())).await;
                }
                move_to_failed(&folder, "embedded").await.ok();
            }
        }
    }
    Ok(())
}
