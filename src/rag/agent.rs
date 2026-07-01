use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{anyhow, Result};
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use reagent_rs::notifications::Notification;
use reagent_rs::prelude::*;
use reagent_rs::AsyncToolFn;

use crate::rag::comm::embedding::EmbeddingVector;
use crate::rag::comm::qdrant::vector_search;
use crate::rag::comm::OllamaEmbeddingClient;
use crate::rag::models::chunks::ResultChunk;
use crate::rag::processing::dedup;

use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};

fn resource_path(relative: &str) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.join(relative)
}

pub async fn build_search_agent(chunks_tx: Sender<Value>) -> Result<(Agent, tokio::sync::mpsc::Receiver<Notification>)> {
    let model = env::var("CHAT_MODEL").unwrap_or_else(|_| "DeepSeek-V4-Flash".into());
    let base_url = env::var("OPENAI_COMPATIBLE_BASE_URL").unwrap_or_else(|_| "http://localhost:11434/v1".into());
    let api_key = env::var("OPENAI_COMPATIBLE_API_KEY").ok();
    let system_prompt_path = resource_path("resources/agent/search_system.txt");
    let prompt_template_path = resource_path("resources/agent/search_prompt.txt");
    let system_prompt = std::fs::read_to_string(&system_prompt_path).map_err(|e| anyhow!("Failed to load system prompt from {:?}: {e}", system_prompt_path))?;
    let prompt_template = Template::from_file(&prompt_template_path).map_err(|e| anyhow!("Failed to load prompt template from {:?}: {e}", prompt_template_path))?;

    println!("[AGENT] Building search agent — model: {model}, base_url: {base_url}, api_key: {}", api_key.is_some());

    let rag_tool = build_rag_tool(chunks_tx)?;

    let mut builder = AgentBuilder::default()
        .set_name("woodstock-search")
        .set_model(&model)
        .set_base_url(&base_url)
        .set_provider(Provider::OpenAi)
        .set_stream(true)
        .set_system_prompt(&system_prompt)
        .set_template(prompt_template)
        .add_tool(rag_tool);

    if let Some(key) = api_key {
        builder = builder.set_api_key(key);
    }

    let start = Instant::now();
    let (agent, rx) = builder
        .build_with_notification()
        .await
        .map_err(|e| anyhow!("Failed to build agent: {e}"))?;
    println!("[AGENT] Agent built in {:?}", start.elapsed());

    Ok((agent, rx))
}

fn build_rag_tool(chunks_tx: Sender<Value>) -> Result<Tool> {
    let embeddings = OllamaEmbeddingClient::default();

    let exec: AsyncToolFn = {
        let embeddings = Arc::new(embeddings);
        Arc::new(move |args: Value| {
            let embeddings = embeddings.clone();
            let chunks_tx = chunks_tx.clone();
            Box::pin(async move {
                let start = Instant::now();

                let query = args
                    .get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        ToolExecutionError::ArgumentParsingError("Missing 'query' argument".into())
                    })?;

                println!("[RAG TOOL] Called with query: \"{query}\"");

                let emb_start = Instant::now();
                let emb_query = GenerateEmbeddingsRequest::new("bge-m3".to_owned(), EmbeddingsInput::Single(query.to_owned()));
                let emb = embeddings.embed(emb_query).await.map_err(|e| {
                    let msg = format!("Embedding failed: {e}");
                    println!("[RAG TOOL] ERROR — {msg}");
                    ToolExecutionError::ExecutionFailed(msg)
                })?;
                println!("[RAG TOOL] Embedding done in {:?} — vector dim: {}", emb_start.elapsed(), emb.embeddings[0].len());

                let vector = EmbeddingVector(emb.embeddings[0].clone());

                let search_start = Instant::now();
                let search_resp = vector_search(vector).await.map_err(|e| {
                    let msg = format!("Vector search failed: {e}");
                    println!("[RAG TOOL] ERROR — {msg}");
                    ToolExecutionError::ExecutionFailed(msg)
                })?;
                println!("[RAG TOOL] Vector search done in {:?}", search_start.elapsed());

                let dedup_start = Instant::now();
                let chunks: Vec<ResultChunk> = dedup(search_resp);
                println!("[RAG TOOL] Dedup done in {:?} — got {} chunks", dedup_start.elapsed(), chunks.len());

                for (i, c) in chunks.iter().enumerate() {
                    println!("  chunk[{}] — doc_id: {}, seq: {}, score: {:.4}", i, c.doc_id, c.doc_seq_num, c.score);
                }

                let chunks_json = serde_json::to_value(&chunks).map_err(|e| {
                    let msg = format!("Serialize chunks failed: {e}");
                    println!("[RAG TOOL] ERROR — {msg}");
                    ToolExecutionError::ExecutionFailed(msg)
                })?;

                let msg = serde_json::json!({
                    "type": "chunks",
                    "value": chunks_json,
                    "display": false,
                });
                let sent = chunks_tx.try_send(msg);
                println!("[RAG TOOL] Chunks sent to stream channel: {:?} — total time: {:?}", sent.map(|_| "ok"), start.elapsed());

                Ok(serde_json::to_string(&chunks).unwrap_or_default())
            })
        })
    };

    ToolBuilder::new()
        .function_name("rag_search")
        .function_description(
            "Searches the document store and returns the most relevant document excerpts with metadata. \
             The query should be formulated as a natural language question (e.g. \"What is the tensile \
             strength of densified wood?\") because the retrieval system matches questions to questions. \
             Call this first to gather evidence before answering.",
        )
        .add_required_property("query", "string", "The search query to find relevant documents")
        .executor(exec)
        .build()
        .map_err(|e| anyhow!("Failed to build RAG tool: {e}"))
}
