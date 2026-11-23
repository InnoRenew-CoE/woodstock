use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use tokio::fs;
use tokio::task::JoinHandle;

pub mod stage_io;
use stage_io::*;

use crate::db;
use crate::rag::{Chunk, ChunkedFile, EmbeddedChunk, HypeChunk, LoadedFile, Rag, RagProcessableFile};

/// How long the scheduler waits before it looks at the stages again.
const POLL_INTERVAL: Duration = Duration::from_secs(5);

/// One stage of the ingest pipeline. A submission sits in exactly one stage at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    New,
    Loaded,
    Chunked,
    Hyped,
    Embedded,
}

impl Stage {
    fn name(self) -> &'static str {
        match self {
            Stage::New => "new",
            Stage::Loaded => "loaded",
            Stage::Chunked => "chunked",
            Stage::Hyped => "hyped",
            Stage::Embedded => "embedded",
        }
    }

    async fn run(self, rag: &Rag) -> Result<StageReport> {
        match self {
            Stage::New => process_new(rag).await,
            Stage::Loaded => process_loaded(rag).await,
            Stage::Chunked => process_chunked(rag).await,
            Stage::Hyped => process_hyped(rag).await,
            Stage::Embedded => process_embedded(rag).await,
        }
    }
}

/// What one pass over a stage did, used for the per pass summary line.
#[derive(Debug, Default)]
struct StageReport {
    processed: usize,
    failed: usize,
}

/// Runs one worker per stage. Each pass restarts the stages that finished, so at most
/// one worker per stage is alive at any time.
pub async fn run(rag: Rag) -> Result<()> {
    println!(
        "[WORKER] starting scheduler over {}, checking the stages every {POLL_INTERVAL:?}",
        staging_dir().display()
    );
    let rag = Arc::new(rag);

    let mut new_worker = spawn_stage(Stage::New, &rag);
    let mut loaded_worker = spawn_stage(Stage::Loaded, &rag);
    let mut chunked_worker = spawn_stage(Stage::Chunked, &rag);
    let mut hyped_worker = spawn_stage(Stage::Hyped, &rag);
    let mut embedded_worker = spawn_stage(Stage::Embedded, &rag);

    loop {
        if new_worker.is_finished() {
            new_worker = spawn_stage(Stage::New, &rag);
        }
        if loaded_worker.is_finished() {
            loaded_worker = spawn_stage(Stage::Loaded, &rag);
        }
        if chunked_worker.is_finished() {
            chunked_worker = spawn_stage(Stage::Chunked, &rag);
        }
        if hyped_worker.is_finished() {
            hyped_worker = spawn_stage(Stage::Hyped, &rag);
        }
        if embedded_worker.is_finished() {
            embedded_worker = spawn_stage(Stage::Embedded, &rag);
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Starts one pass over a stage in its own task.
fn spawn_stage(stage: Stage, rag: &Arc<Rag>) -> JoinHandle<()> {
    let rag = Arc::clone(rag);
    tokio::spawn(async move { run_stage(stage, rag).await })
}

/// Runs a single pass over one stage and logs how it went.
async fn run_stage(stage: Stage, rag: Arc<Rag>) {
    let started = Instant::now();
    match stage.run(&rag).await {
        // An empty pass is the normal case, keep it out of the log.
        Ok(report) if report.processed == 0 => {}
        Ok(report) => println!(
            "[WORKER][{}] worker finished in {:.2}s: {} submission(s) processed, {} failed",
            stage.name(),
            elapsed_secs(started),
            report.processed,
            report.failed
        ),
        Err(error) => eprintln!("[WORKER][{}] worker failed after {:.2}s: {error:#}", stage.name(), elapsed_secs(started)),
    }
}

fn elapsed_secs(started: Instant) -> f64 {
    started.elapsed().as_secs_f64()
}

/// Writes the ingest status of a submission to the database and logs the outcome.
async fn set_file_status(stage: Stage, uuid: &str, status: &str, message: Option<&str>) {
    let metadata_file = metadata_path(stage.name(), uuid);
    let meta = match read_json::<RagProcessableFile>(&metadata_file).await {
        Ok(meta) => meta,
        Err(error) => {
            eprintln!(
                "[WORKER][{}] {uuid}: cannot read {} for the status update: {error:#}",
                stage.name(),
                metadata_file.display()
            );
            return;
        }
    };
    let client = db::build_db_client().await;
    match db::update_file_status(&client, &meta.internal_id, status, message).await {
        Ok(rows) => println!(
            "[WORKER][{}] {uuid}: internal id {} set to {status} ({rows} row(s))",
            stage.name(),
            meta.internal_id
        ),
        Err(error) => eprintln!(
            "[WORKER][{}] {uuid}: cannot set internal id {} to {status}: {error}",
            stage.name(),
            meta.internal_id
        ),
    }
}

/// Moves a failed submission into `{stage}/failed/{uuid}` and logs both outcomes.
async fn quarantine(stage: Stage, uuid: &str, folder: &PathBuf) {
    match move_to_failed(folder, stage.name()).await {
        Ok(()) => eprintln!("[WORKER][{}] {uuid}: moved to {}/failed", stage.name(), stage.name()),
        Err(error) => eprintln!("[WORKER][{}] {uuid}: cannot move to {}/failed: {error:#}", stage.name(), stage.name()),
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

async fn process_new(rag: &Rag) -> Result<StageReport> {
    let stage = Stage::New;
    let uuids = list_submissions(stage.name()).await?;
    if uuids.is_empty() {
        return Ok(StageReport::default());
    }
    println!(
        "[WORKER][new] {} pending submission(s) in {}",
        uuids.len(),
        staging_dir().join(stage.name()).display()
    );

    let mut report = StageReport::default();
    for uuid in uuids {
        report.processed += 1;
        let folder = staging_dir().join(stage.name()).join(&uuid);
        let metadata_file = metadata_path(stage.name(), &uuid);
        let metadata = match read_json::<RagProcessableFile>(&metadata_file).await {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("[WORKER][new] {uuid}: cannot read {}: {error:#}", metadata_file.display());
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
                continue;
            }
        };

        println!(
            "[WORKER][new] {uuid}: loading {} ({:?}) for internal id {}, staged file {}",
            metadata.original_name,
            metadata.file_type,
            metadata.internal_id,
            metadata.path.display()
        );

        let started = Instant::now();
        match rag.insert_meta(&metadata).await {
            Ok(loaded) => {
                println!(
                    "[WORKER][new] {uuid}: parsed in {:.2}s, {} bytes of text, {} image(s)",
                    elapsed_secs(started),
                    loaded.content.len(),
                    loaded.images.len()
                );
                if let Err(error) = write_json(&loaded_path(stage.name(), &uuid), &loaded).await {
                    eprintln!(
                        "[WORKER][new] {uuid}: cannot write {}: {error:#}",
                        loaded_path(stage.name(), &uuid).display()
                    );
                    report.failed += 1;
                    quarantine(stage, &uuid, &folder).await;
                    continue;
                }
                let to = staging_dir().join(Stage::Loaded.name()).join(&uuid);
                match move_submission(&folder, &to).await {
                    Ok(()) => println!("[WORKER][new] {uuid}: new -> loaded"),
                    Err(error) => {
                        eprintln!("[WORKER][new] {uuid}: cannot move new -> loaded ({}): {error:#}", to.display());
                        report.failed += 1;
                    }
                }
            }
            Err(error) => {
                eprintln!("[WORKER][new] {uuid}: parsing failed after {:.2}s: {error:#}", elapsed_secs(started));
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
            }
        }
    }
    Ok(report)
}

async fn process_loaded(_rag: &Rag) -> Result<StageReport> {
    let stage = Stage::Loaded;
    let uuids = list_submissions(stage.name()).await?;
    if uuids.is_empty() {
        return Ok(StageReport::default());
    }
    println!("[WORKER][loaded] {} pending submission(s)", uuids.len());

    let mut report = StageReport::default();
    for uuid in uuids {
        report.processed += 1;
        let folder = staging_dir().join(stage.name()).join(&uuid);
        let loaded_file = loaded_path(stage.name(), &uuid);
        let loaded = match read_json::<LoadedFile>(&loaded_file).await {
            Ok(loaded) => loaded,
            Err(error) => {
                eprintln!("[WORKER][loaded] {uuid}: cannot read {}: {error:#}", loaded_file.display());
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
                continue;
            }
        };

        let text_bytes = loaded.content.len();
        let started = Instant::now();
        let chunked = Rag::insert_chunk(loaded);
        println!(
            "[WORKER][loaded] {uuid}: split {} bytes into {} chunk(s) in {:.2}s",
            text_bytes,
            chunked.chunks.len(),
            elapsed_secs(started)
        );

        let chunked_file = chunked_path(stage.name(), &uuid);
        if let Err(error) = write_json(&chunked_file, &chunked).await {
            eprintln!("[WORKER][loaded] {uuid}: cannot write {}: {error:#}", chunked_file.display());
            report.failed += 1;
            quarantine(stage, &uuid, &folder).await;
            continue;
        }
        let to = staging_dir().join(Stage::Chunked.name()).join(&uuid);
        match move_submission(&folder, &to).await {
            Ok(()) => println!("[WORKER][loaded] {uuid}: loaded -> chunked"),
            Err(error) => {
                eprintln!("[WORKER][loaded] {uuid}: cannot move loaded -> chunked ({}): {error:#}", to.display());
                report.failed += 1;
            }
        }
    }
    Ok(report)
}

async fn process_chunked(rag: &Rag) -> Result<StageReport> {
    let stage = Stage::Chunked;
    let uuids = list_submissions(stage.name()).await?;
    if uuids.is_empty() {
        return Ok(StageReport::default());
    }
    println!("[WORKER][chunked] {} pending submission(s)", uuids.len());

    let mut report = StageReport::default();
    for uuid in uuids {
        report.processed += 1;
        let folder = staging_dir().join(stage.name()).join(&uuid);
        let chunked_file = chunked_path(stage.name(), &uuid);
        let chunked = match read_json::<ChunkedFile<Chunk>>(&chunked_file).await {
            Ok(chunked) => chunked,
            Err(error) => {
                eprintln!("[WORKER][chunked] {uuid}: cannot read {}: {error:#}", chunked_file.display());
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
                continue;
            }
        };
        let chunk_count = chunked.chunks.len();

        let started = Instant::now();
        match rag.insert_hype(chunked).await {
            Ok(hyped) => {
                let with_questions = hyped.chunks.iter().filter(|chunk| !chunk.questions.is_empty()).count();
                let question_count: usize = hyped.chunks.iter().map(|chunk| chunk.questions.len()).sum();
                println!(
                    "[WORKER][chunked] {uuid}: generated {} question(s) for {with_questions}/{chunk_count} chunk(s) in {:.2}s",
                    question_count,
                    elapsed_secs(started)
                );
                if with_questions < chunk_count {
                    eprintln!(
                        "[WORKER][chunked] {uuid}: {} of {chunk_count} chunk(s) got no questions",
                        chunk_count - with_questions
                    );
                }

                let hyped_file = hyped_path(stage.name(), &uuid);
                if let Err(error) = write_json(&hyped_file, &hyped).await {
                    eprintln!("[WORKER][chunked] {uuid}: cannot write {}: {error:#}", hyped_file.display());
                    report.failed += 1;
                    quarantine(stage, &uuid, &folder).await;
                    continue;
                }
                let to = staging_dir().join(Stage::Hyped.name()).join(&uuid);
                match move_submission(&folder, &to).await {
                    Ok(()) => println!("[WORKER][chunked] {uuid}: chunked -> hyped"),
                    Err(error) => {
                        eprintln!("[WORKER][chunked] {uuid}: cannot move chunked -> hyped ({}): {error:#}", to.display());
                        report.failed += 1;
                    }
                }
            }
            Err(error) => {
                eprintln!(
                    "[WORKER][chunked] {uuid}: question generation failed after {:.2}s: {error:#}",
                    elapsed_secs(started)
                );
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
            }
        }
    }
    Ok(report)
}

async fn process_hyped(rag: &Rag) -> Result<StageReport> {
    let stage = Stage::Hyped;
    let uuids = list_submissions(stage.name()).await?;
    if uuids.is_empty() {
        return Ok(StageReport::default());
    }
    println!("[WORKER][hyped] {} pending submission(s)", uuids.len());

    let mut report = StageReport::default();
    for uuid in uuids {
        report.processed += 1;
        let folder = staging_dir().join(stage.name()).join(&uuid);
        let hyped_file = hyped_path(stage.name(), &uuid);
        let hyped = match read_json::<ChunkedFile<HypeChunk>>(&hyped_file).await {
            Ok(hyped) => hyped,
            Err(error) => {
                eprintln!("[WORKER][hyped] {uuid}: cannot read {}: {error:#}", hyped_file.display());
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
                continue;
            }
        };
        let chunk_count = hyped.chunks.len();

        let started = Instant::now();
        match rag.insert_embed(hyped).await {
            Ok(embedded_chunks) => {
                let point_count = embedded_chunks.len();
                println!(
                    "[WORKER][hyped] {uuid}: embedded {point_count} point(s) from {chunk_count} chunk(s) in {:.2}s",
                    elapsed_secs(started)
                );
                if point_count < chunk_count {
                    eprintln!("[WORKER][hyped] {uuid}: only {point_count} point(s) for {chunk_count} chunk(s), the rest will not be indexed");
                }

                let embedded_file = folder.join("embedded.json");
                if let Err(error) = write_json(&embedded_file, &embedded_chunks).await {
                    eprintln!("[WORKER][hyped] {uuid}: cannot write {}: {error:#}", embedded_file.display());
                    report.failed += 1;
                    quarantine(stage, &uuid, &folder).await;
                    continue;
                }
                let to = staging_dir().join(Stage::Embedded.name()).join(&uuid);
                match move_submission(&folder, &to).await {
                    Ok(()) => println!("[WORKER][hyped] {uuid}: hyped -> embedded"),
                    Err(error) => {
                        eprintln!("[WORKER][hyped] {uuid}: cannot move hyped -> embedded ({}): {error:#}", to.display());
                        report.failed += 1;
                    }
                }
            }
            Err(error) => {
                eprintln!("[WORKER][hyped] {uuid}: embedding failed after {:.2}s: {error:#}", elapsed_secs(started));
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
            }
        }
    }
    Ok(report)
}

async fn process_embedded(_rag: &Rag) -> Result<StageReport> {
    let stage = Stage::Embedded;
    let uuids = list_submissions(stage.name()).await?;
    if uuids.is_empty() {
        return Ok(StageReport::default());
    }
    println!("[WORKER][embedded] {} pending submission(s)", uuids.len());

    let mut report = StageReport::default();
    for uuid in uuids {
        report.processed += 1;
        let folder = staging_dir().join(stage.name()).join(&uuid);
        let embedded_file = folder.join("embedded.json");
        let embedded_chunks = match read_json::<Vec<EmbeddedChunk>>(&embedded_file).await {
            Ok(embedded_chunks) => embedded_chunks,
            Err(error) => {
                eprintln!("[WORKER][embedded] {uuid}: cannot read {}: {error:#}", embedded_file.display());
                report.failed += 1;
                quarantine(stage, &uuid, &folder).await;
                continue;
            }
        };
        let point_count = embedded_chunks.len();
        println!("[WORKER][embedded] {uuid}: upserting {point_count} point(s) to qdrant");

        let started = Instant::now();
        match Rag::insert_qdrant(embedded_chunks).await {
            Ok(()) => {
                println!("[WORKER][embedded] {uuid}: qdrant upsert finished in {:.2}s", elapsed_secs(started));
                set_file_status(stage, &uuid, "done", None).await;
                match fs::remove_dir_all(&folder).await {
                    Ok(()) => println!("[WORKER][embedded] {uuid}: staged folder {} removed", folder.display()),
                    Err(error) => eprintln!("[WORKER][embedded] {uuid}: cannot remove {}: {error}", folder.display()),
                }
            }
            Err(error) => {
                eprintln!(
                    "[WORKER][embedded] {uuid}: qdrant upsert of {point_count} point(s) failed after {:.2}s: {error:#}",
                    elapsed_secs(started)
                );
                report.failed += 1;
                set_file_status(stage, &uuid, "failed", Some(&error.to_string())).await;
                quarantine(stage, &uuid, &folder).await;
            }
        }
    }
    Ok(report)
}
