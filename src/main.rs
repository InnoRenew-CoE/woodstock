use std::fs;
use std::io::Write;
use std::time::Instant;

use chrono::NaiveDateTime;
use anyhow::Result;
use serde_json::json;

mod shared;
mod rag;

use rag::Rag;
use shared::file::{Answer, WoodstockFileData};
use shared::file_type::FileType;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables (if present)
    if let Err(e) = dotenv::dotenv() {
        return Err(e.into());
    }

    let rag = Rag::default();

    // Directory containing files to process
    let input_dir = "./resources/wood";
    let done_dir = "./resources/done";
    let failed_dir = "./resources/failed";

    // Create output directories if they don't exist
    fs::create_dir_all(done_dir)?;
    fs::create_dir_all(failed_dir)?;

    // Iterate over each file in `./resources/wood`
    for (id, entry) in fs::read_dir(input_dir)?.enumerate() {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Grab file name for logging/moving
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                eprintln!("Skipping file with no valid name: {:?}", path);
                continue;
            }
        };

        // Attempt to guess file type from extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let file_type = match extension.as_str() {
            "pdf" => FileType::Pdf,
            "md"  => FileType::Markdown,
            "txt" => FileType::Text,
            // Unknown extension — skip or handle differently
            _ => {
                eprintln!("Skipping unsupported file: {:?}", file_name);
                continue;
            }
        };

        // Build a simple WoodstockFileData.  
        // Customize as needed: date_of_submission, answers, etc.
        let woodstock_data = WoodstockFileData {
            path: path.display().to_string(),
            internal_id: id as i64,
            original_name: file_name.clone(),
            answers: vec![
                // Possibly empty, or real data read from somewhere
                Answer {
                    question_id: 1,
                    value: json!("Auto-generated answer #1"),
                },
            ],
            tags: Some(vec!["auto".to_string()]),
            file_type,
            submitted_by: 1234,
            date_of_submission: NaiveDateTime::from_timestamp_opt(1736784000, 0)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
        };

        // Record the start time
        let start_time = Instant::now();

        // Attempt to insert
        match rag.insert(woodstock_data).await {
            Ok(_) => {
                // Successfully inserted
                let duration = start_time.elapsed();
                println!(
                    "Successfully inserted file '{}' in {:?}",
                    file_name, duration
                );

                // Move file to `./resources/done/`
                let done_path = format!("{}/{}", done_dir, file_name);
                if let Err(e) = fs::rename(&path, &done_path) {
                    eprintln!("Failed to move '{}' to done: {}", file_name, e);
                }
            }
            Err(e) => {
                // Insert failed — log the error and move file to `failed` folder
                let duration = start_time.elapsed();
                eprintln!(
                    "Failed to insert file '{}' in {:?}: {:?}",
                    file_name, duration, e
                );

                // Move to `./resources/failed/`
                let failed_path = format!("{}/{}", failed_dir, file_name);
                if let Err(move_err) = fs::rename(&path, &failed_path) {
                    eprintln!("Failed to move '{}' to failed: {}", file_name, move_err);
                }

                // Write an error log (same name but `.txt`)
                let log_file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let error_log_path = format!("{}/{}.txt", failed_dir, log_file_stem);

                match fs::File::create(&error_log_path) {
                    Ok(mut f) => {
                        let _ = writeln!(
                            f,
                            "Failed to insert file '{}': {}",
                            file_name, e
                        );
                    }
                    Err(e2) => {
                        eprintln!("Could not create error log '{}': {}", error_log_path, e2);
                    }
                }
            }
        }
    }

    Ok(())
}
