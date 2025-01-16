use crate::rag::{comm::{question::Question, OllamaClient}, models::{chunks::Chunk, ChunkedFile}};

pub async fn summarize_document(file: &ChunkedFile<Chunk>, ollama: &OllamaClient) -> String {
    let summary_prompts = generate_prompts(&file);
    let chunk_summaries = ollama.answer_all(summary_prompts).await;
    create_document_summary(chunk_summaries, file.original_file_description.clone(), ollama).await
}


async fn create_document_summary(chunk_summaries: Vec<String>, original_doc_summary: Option<String>, ollama: &OllamaClient) -> String {
    let mut context = chunk_summaries;
    if let Some(summary) = original_doc_summary {
        context.push(summary);
    }
    match ollama
        .generate(Question::from("Summarize this document in context into 3 sentances.").set_context(context))
        .await {
            Ok(r) => r.response,
            Err(_) => "".into(),
        }
}


fn generate_prompts(file: &ChunkedFile<Chunk>) -> Vec<Question> {
    let system_prompt = "You are the best summarizer language model out there.";
    let question = "Given a context paragraph wirite one sentance that best \
        captures what the context is describing";
    
    file
        .chunks
        .iter()
        .map(|c| Question::from(question)
            .set_system_prompt(&system_prompt)
            .set_context(vec![c.text.clone()])
        )
        .collect()
}
