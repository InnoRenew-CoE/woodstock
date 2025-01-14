use ollama_rs::{error::OllamaError, generation::completion::GenerationResponse};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use futures::future::join_all;
use regex::RegexBuilder;

use crate::rag::comm::{embedding::Embeddable, question::{self, Question}, OllamaClient};

use super::{chunk::Chunk, chunked_file::ChunkedFile};

#[derive(Debug)]
pub struct HypeChunk {
    pub id: i32,
    pub text: String,
    pub questions: Vec<String>
}

impl From<&Chunk> for HypeChunk {
    fn from(value: &Chunk) -> Self {
        Self { id: value.id, text: value.text.clone(), questions: vec![] }
    }
}

impl HypeChunk {
    pub fn set_questions(mut self, questions: Vec<String>) -> Self {
        self.questions = questions;
        self
    }
}

impl Embeddable for HypeChunk {

}

pub async fn hype(file: ChunkedFile<Chunk>, ollama: &OllamaClient) -> ChunkedFile<HypeChunk> {
    let summary_prompts = generate_questions(&file);
    println!("Questions generated...");
    let chunk_summaries = answer_all(summary_prompts, ollama).await;
    let summary = create_document_summary(chunk_summaries, ollama).await;
    println!("Summary: {:#?}", summary);

    let hype_question_prompts = generate_hype_prompt_questions(summary, &file);
    println!("Qs: {:#?}", hype_question_prompts);

    let hype_questions = answer_all(hype_question_prompts, ollama).await;
    let hype_chunks = generate_hype_chunks(&file.chunks, hype_questions);
    replace_chunks(file, hype_chunks)
}

async fn create_document_summary(chunk_summaries: Vec<String>, ollama: &OllamaClient) -> String {
    match ollama
        .generate(Question::from("Summarize this document.").set_context(vec![chunk_summaries.join(" ")]))
        .await {
            Ok(r) => r.response,
            Err(_) => "".into(),
        }
}

fn replace_chunks(file: ChunkedFile<Chunk>, hype_chunks: Vec<HypeChunk>) -> ChunkedFile<HypeChunk> {
    let ChunkedFile {
        file_type,
        chunks: _,
        internal_id,
        answers,
        tags,
    } = file;

    ChunkedFile {
        file_type,
        chunks: hype_chunks,
        internal_id,
        answers,
        tags,
    }
}

fn generate_hype_chunks(chunks: &[Chunk], hype_questions: Vec<String>) -> Vec<HypeChunk> {    
    let list_pattern = RegexBuilder::new(r"^\s*[\-\*]|\s*\d+\.\s*|\s*[a-zA-Z]\)\s*|\s*\(\d+\)\s*|\s*\([a-zA-Z]\)\s*|\s*\([ivxlcdm]+\)\s*")
        .case_insensitive(true)
        .build()
        .unwrap();
    
    let mut hype_chunks = vec![];
    for (i, chunk) in chunks.into_iter().enumerate() {
        let questions: Vec<String> = hype_questions[i]
            .split('\n')
            .map(|line| {
                let without_pattern = list_pattern.replace(line, "");
                without_pattern.trim().to_string()
            })
            .filter(|cleaned_line| !cleaned_line.is_empty())
            .collect();

        let hype_chunk = HypeChunk::from(chunk).set_questions(questions);
        hype_chunks.push(hype_chunk);
    }
    hype_chunks
}

fn generate_hype_prompt_questions(summary: String, file: &ChunkedFile<Chunk>) -> Vec<Question> {
    let question = "With the help of a summary, analyze the context text and generate essential questions that, when answered, capture the main points and core meaning of the text. \
        The questions should be exhaustive and understandable without context. When possible, named entities should be referenced by their full name. \
        However add questions that are diverse in topic. \
        It is extremely important that you only answer with questions and each question should be written in its own line (separated by newline) with no prefix.";
    let system_prompt = "You are an agent specialized to only answer in form of questions.";

    file
        .chunks
        .iter()
        .map(|c| Question::from(question)
            .set_system_prompt(&system_prompt)
            .set_context(vec![format!(
                "DOCUMENT:\n{}\n\nCONTEXT TEXT:\n{}",
                summary,
                c.text
            )])
        )
        .collect()
}


fn generate_questions(file: &ChunkedFile<Chunk>) -> Vec<Question> {
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

async fn answer_all(questions: Vec<Question>, ollama: &OllamaClient) -> Vec<String> {
    let futures = questions.into_iter().map(|q| async move {
        eprintln!("Starting question {:#?}", q);
        let now = std::time::Instant::now();
        let response = ollama.generate(q.clone()).await.ok();
        eprintln!("Finished question {:#?}; took {:?}", q, now.elapsed());
        response
    });

    let results = futures::future::join_all(futures).await;
    results.into_iter()
        .map(|r| r.map_or_else(|| "".to_owned(), |resp| resp.response))
        .collect()
}
