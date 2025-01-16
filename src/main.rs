use std::fs;
use std::io::Write;
use std::time::Instant;
use tokio::io::{self, AsyncWriteExt};
use tokio_stream::StreamExt;

use anyhow::Result;

mod shared;
mod rag;

use rag::{Rag, RagProcessableFile};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables (if present)
    if let Err(e) = dotenv::dotenv() {
        return Err(e.into());
    }

    let rag = Rag::default();

    if let Err(e) = prompt(&rag, "When did academic publications about wood densification first appear?").await {
        println!("Something went wrong with prompt: {:#?}", e);
    }
   
    Ok(())
}


async fn prompt(rag: &Rag, question: &str) -> Result<()> {
    let mut result = rag.search(question.into()).await?;
    let mut stdout = io::stdout();
    while let Some(res) = result.stream.next().await {
        let responses = res.unwrap();
        for resp in responses {
            stdout.write_all(resp.response.as_bytes()).await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
    Ok(())
}

async fn embed_all(rag: &Rag) -> Result<()> {
    let input_dir = "./resources/wood";
    let done_dir = "./resources/done";
    let failed_dir = "./resources/failed";

    fs::create_dir_all(done_dir)?;
    fs::create_dir_all(failed_dir)?;

    for (id, entry) in fs::read_dir(input_dir)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
                continue;
            }
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                eprintln!("Skipping file with no valid name: {:?}", path);
                continue;
            }
        };
        
        let woodstock_data = RagProcessableFile {
                path: path.clone(),
                internal_id: id.to_string(),
                original_name: file_name.clone(),
                tags: Some(vec!["auto".to_string()]),
                file_description: None,
        };
        let start_time = Instant::now();
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