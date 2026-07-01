use anyhow::{Context, Result};

use crate::rag::comm::{ChatClient, ImageInput};
use crate::rag::comm::question::Question;
use crate::rag::models::{chunks::HypeChunk, ImageRef};

pub async fn add_image_hype_questions(chunks: &mut [HypeChunk], llm: &ChatClient) {
    for chunk in chunks.iter_mut() {
        for image in chunk.images.clone() {
            match generate_questions(llm, chunk, &image).await {
                Ok(questions) => chunk.questions.extend(questions),
                Err(err) => eprintln!("[IMAGE_HYPE] {} failed: {err}", image.route),
            }
        }
    }
}

async fn generate_questions(llm: &ChatClient, chunk: &HypeChunk, image: &ImageRef) -> Result<Vec<String>> {
    let image_bytes = tokio::fs::read(&image.path)
        .await
        .with_context(|| format!("read image for image hype {}", image.path.display()))?;
    let mime_type = mime_guess::from_path(&image.path)
        .first_or_octet_stream()
        .essence_str()
        .to_string();

    let prompt = Question::from(
        "/no_think Generate retrieval questions that can be answered from the image and its surrounding document context. \
         Only answer with questions, one per line, with no numbering or prefixes."
    )
    .set_system_prompt("You generate concise search questions from document images.")
    .set_context(vec![format!(
        "IMAGE ROUTE: {}\nALT TEXT: {}\nDOCUMENT CHUNK CONTEXT:\n{}",
        image.route,
        image.alt_text.clone().unwrap_or_default(),
        chunk.text
    )]);

    let response = llm
        .generate_with_image(
            prompt,
            ImageInput {
                bytes: image_bytes,
                mime_type,
            },
        )
        .await?;

    Ok(clean_questions(response.content.lines().map(|line| line.to_string()).collect()))
}

fn clean_questions(questions: Vec<String>) -> Vec<String> {
    questions
        .into_iter()
        .map(|question| question.trim().trim_start_matches('-').trim().to_string())
        .filter(|question| !question.is_empty())
        .collect()
}
