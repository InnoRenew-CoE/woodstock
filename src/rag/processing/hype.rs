use once_cell::sync::Lazy;
use regex::RegexBuilder;

use crate::rag::comm::question::Question;
use crate::rag::comm::OllamaClient;
use crate::rag::models::chunks::{Chunk, HypeChunk};
use crate::rag::models::ChunkedFile;

static LIST_PATTERN: Lazy<regex::Regex> = Lazy::new(|| {
    RegexBuilder::new(r"^\s*[\-\*]|\s*\d+\.\s*|\s*[a-zA-Z]\)\s*|\s*\(\d+\)\s*|\s*\([ivxlcdm]+\)\s*")
        .case_insensitive(true)
        .build()
        .unwrap()
});

pub async fn hype(file: ChunkedFile<Chunk>, ollama: &OllamaClient) -> ChunkedFile<HypeChunk> {
    // let summary = summarize_document(&file, ollama).await;
    let hype_question_prompts = generate_hype_prompt_questions(&file);
    let hype_questions = ollama.answer_all(hype_question_prompts).await;
    print!("Generated hype questions for {} chunks\n", hype_questions.len());
    let hype_chunks = generate_hype_chunks(&file.chunks, hype_questions);
    replace_chunks(file, hype_chunks)
}

fn replace_chunks(file: ChunkedFile<Chunk>, hype_chunks: Vec<HypeChunk>) -> ChunkedFile<HypeChunk> {
    let ChunkedFile {
        file_type,
        chunks: _,
        internal_id,
        tags,
        original_file_description,
        syntetic_file_description,
    } = file;

    ChunkedFile {
        file_type,
        chunks: hype_chunks,
        internal_id,
        tags,
        original_file_description,
        syntetic_file_description,
    }
}

fn generate_hype_chunks(chunks: &[Chunk], hype_questions: Vec<String>) -> Vec<HypeChunk> {
    let mut hype_chunks = vec![];
    for (i, (chunk, answer)) in chunks.iter().zip(hype_questions.into_iter()).enumerate() {
        let questions: Vec<String> = answer
            .split('\n')
            .map(|line| {
                let without_pattern = LIST_PATTERN.replace(line, "");
                without_pattern.trim().to_string()
            })
            .filter(|cleaned_line| !cleaned_line.is_empty())
            .collect();

        if questions.is_empty() {
            println!("No questions generated for chunk {}", i);
        }
        hype_chunks.push(HypeChunk::from(chunk).set_questions(questions));
    }

    hype_chunks
}

fn generate_hype_prompt_questions(file: &ChunkedFile<Chunk>) -> Vec<Question> {
    let question = "/no_think You will be given a passage from a document.\n Your task is to analyze the context text (passage) and \
        generate essential questions that, when answered, capture the main points and core meaning of the text. \
        The questions should be exhaustive and understandable without context. When possible, named entities should be referenced by their full name. \
        Dont't refer to the text as 'the text' or 'the passage' and don't refer to outside material like 'Figure 2'. \
        However add questions that are diverse in topic. \
        It is extremely important that you only answer with questions and each question should be written in its own line (separated by newline) with no prefix.\
        And finally the answer to each question has to be found in the passage.".to_string();

    let system_prompt = "You are an agent specialized to only answer in list of questions.";
    println!("Generating hype questions prompts for {} chunks", file.chunks.len());
    file.chunks
        .iter()
        .map(|c| {
            Question::from(question.clone())
                .set_system_prompt(&system_prompt)
                .set_context(vec![format!("\nCONTEXT PASSAGE:\n{}", c.text)])
        })
        .collect()
}
