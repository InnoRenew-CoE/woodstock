use std::env;
use std::sync::Arc;

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

pub async fn build_search_agent(chunks_tx: Sender<Value>) -> Result<(Agent, tokio::sync::mpsc::Receiver<Notification>)> {
    let model = env::var("CHAT_MODEL").unwrap_or_else(|_| "DeepSeek-V4-Flash".into());
    let base_url = env::var("OPENAI_COMPATIBLE_BASE_URL").unwrap_or_else(|_| "http://localhost:11434/v1".into());
    let api_key = env::var("OPENAI_COMPATIBLE_API_KEY").ok();
    let system_prompt = std::fs::read_to_string("resources/agent/search_system.txt").map_err(|e| anyhow!("Failed to load system prompt: {e}"))?;
    let prompt_template = Template::from_file("resources/agent/search_prompt.txt").map_err(|e| anyhow!("Failed to load prompt template: {e}"))?;

    let rag_tool = build_rag_tool(chunks_tx)?;

    let mut builder = AgentBuilder::default()
        .set_name("woodstock-search")
        .set_model(&model)
        .set_base_url(&base_url)
        .set_provider(Provider::OpenAi)
        .set_stream(true)
        .set_system_prompt(system_prompt)
        .set_template(prompt_template)
        .add_tool(rag_tool);

    if let Some(key) = api_key {
        builder = builder.set_api_key(key);
    }

    let (agent, rx) = builder
        .build_with_notification()
        .await
        .map_err(|e| anyhow!("Failed to build agent: {e}"))?;

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
                let query = args
                    .get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolExecutionError::ArgumentParsingError("Missing 'query' argument".into()))?;

                let emb_query = GenerateEmbeddingsRequest::new("bge-m3".to_owned(), EmbeddingsInput::Single(query.to_owned()));
                let emb = embeddings
                    .embed(emb_query)
                    .await
                    .map_err(|e| ToolExecutionError::ExecutionFailed(format!("Embedding failed: {e}")))?;
                let vector = EmbeddingVector(emb.embeddings[0].clone());

                let search_resp = vector_search(vector)
                    .await
                    .map_err(|e| ToolExecutionError::ExecutionFailed(format!("Vector search failed: {e}")))?;

                let chunks: Vec<ResultChunk> = dedup(search_resp);

                let chunks_json =
                    serde_json::to_value(&chunks).map_err(|e| ToolExecutionError::ExecutionFailed(format!("Serialize chunks failed: {e}")))?;

                let msg = serde_json::json!({
                    "type": "chunks",
                    "value": chunks_json,
                    "display": false,
                });
                let _ = chunks_tx.try_send(msg);

                Ok(serde_json::to_string(&chunks).unwrap_or_default())
            })
        })
    };

    ToolBuilder::new()
        .function_name("rag_search")
        .function_description(
            "Searches the document store for information relevant to the user's query. \
             Returns a list of document excerpts with metadata. Call this first before answering.",
        )
        .add_required_property("query", "string", "The search query to find relevant documents")
        .executor(exec)
        .build()
        .map_err(|e| anyhow!("Failed to build RAG tool: {e}"))
}
