use crate::rag::{
    comm::{question::Question, ChatClient},
    models::{chunks::ResultChunk, SearchResult},
};
use anyhow::Result;

pub async fn prompt(prompt: String, chunks: Vec<ResultChunk>, llm: &ChatClient) -> Result<SearchResult> {
    let llm_prompt = construct_prompt(prompt, &chunks);
    println!("Prompt: {:#?}", llm_prompt);
    let stream = llm.generate_stream(llm_prompt).await?;
    Ok(SearchResult { chunks, stream })
}

fn construct_prompt(prompt: String, chunks: &Vec<ResultChunk>) -> Question {
    let system_message = "/no_think You answer questions using only the provided document excerpts. \
        Be precise, grounded, and concise. If the excerpts do not contain enough information, say so \
        instead of guessing. When useful, mention which document or metadata supports the answer."
        .to_string();

    let context = chunks
        .iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let chunk_context: String = chunk.into();
            format!("Excerpt {}:\n{}", idx + 1, chunk_context.trim())
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let question = format!(
        r#"
Retrieved document excerpts:

{}

User question:
{}

Answer requirements:
- Respond in markdown.
- Use only facts supported by the retrieved excerpts.
- Prefer a direct answer first, then brief supporting details.
- If excerpts conflict, describe the conflict.
- If the answer is not present in the excerpts, say that the available documents do not answer it.

    "#,
        context, prompt
    );

    Question::from(question).set_system_prompt(&system_message)
}
