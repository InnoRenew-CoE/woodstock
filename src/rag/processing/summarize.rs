use crate::rag::{
    comm::{question::Question, ChatClient},
    models::{chunks::Chunk, ChunkedFile},
};

pub async fn summarize_document(file: &ChunkedFile<Chunk>, llm: &ChatClient) -> String {
    let summary_prompts = generate_prompts(&file);
    let chunk_summaries = llm.answer_all(summary_prompts).await;
    create_document_summary(chunk_summaries, file.original_file_description.clone(), llm).await
}

async fn create_document_summary(chunk_summaries: Vec<String>, original_doc_summary: Option<String>, llm: &ChatClient) -> String {
    let mut context = chunk_summaries;
    if let Some(summary) = original_doc_summary {
        context.push(summary);
    }
    match llm
        .generate(Question::from("Summarize this document in context into 3 sentances.").set_context(context))
        .await
    {
        Ok(r) => r.content,
        Err(_) => "".into(),
    }
}

fn generate_prompts(file: &ChunkedFile<Chunk>) -> Vec<Question> {
    let system_prompt = "/no_think You are the best summarizer language model out there.";
    let question = "Given a context paragraph wirite one sentance that best \
        captures what the context is describing";

    file.chunks
        .iter()
        .map(|c| {
            Question::from(question)
                .set_system_prompt(&system_prompt)
                .set_context(vec![c.text.clone()])
        })
        .collect()
}
