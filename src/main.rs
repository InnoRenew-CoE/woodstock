// use rag::comm::ollama::OllamaClient;

use actix_web::{web, App, HttpServer, Route};
use actix_web_lab::web::spa;
use anyhow::Result;
use chrono::NaiveDateTime;
use rag::Rag;
use serde_json::json;
use shared::file::{Answer, WoodstockFileData};

pub mod rag;
mod server;
pub mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = dotenv::dotenv() {
        return Err(e.into());
    }

    let rag = Rag::default();

    let dummy_instance = WoodstockFileData {
        path: "./resources/rules.md".to_string(),
        internal_id: 42,
        original_name: "rules.md".to_string(),
        answers: vec![
            Answer {
                question_id: 1,
                value: json!("Yes, the rule applies."),
            },
            Answer {
                question_id: 2,
                value: json!({
                    "summary": "Rules at the university must be followed strictly.",
                    "details": [
                        "No cheating",
                        "No plagiarism",
                        "Respect deadlines"
                    ]
                }),
            },
        ],
        tags: Some(vec!["university".to_string(), "rules".to_string()]),
        file_type: shared::file_type::FileType::Markdown,
        submitted_by: 1001,
        date_of_submission: NaiveDateTime::from_timestamp_opt(1736784000, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
    };

    let dummy_instance = WoodstockFileData {
        path: "./resources/slides.pdf".to_string(),
        internal_id: 43,
        original_name: "slides.pdf".to_string(),
        answers: vec![
            Answer {
                question_id: 1,
                value: json!("Yes, the rule applies."),
            },
            Answer {
                question_id: 2,
                value: json!({
                    "summary": "Rules at the university must be followed strictly.",
                    "details": [
                        "No cheating",
                        "No plagiarism",
                        "Respect deadlines"
                    ]
                }),
            },
        ],
        tags: Some(vec!["university".to_string(), "rules".to_string()]),
        file_type: shared::file_type::FileType::Pdf,
        submitted_by: 1001,
        date_of_submission: NaiveDateTime::from_timestamp_opt(1736784000, 0)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
    };

    println!("{:#?}", rag.insert(dummy_instance));

    server::start_server().await;
    Ok(())
}

// TODO: Static Files:      https://github.com/actix/examples/blob/master/basics/static-files/src/main.rs
// TODO: Nested routing:    https://github.com/actix/examples/tree/master/basics/nested-routing
// TODO: Postgres usage:    https://github.com/actix/examples/tree/master/databases/postgres/src
