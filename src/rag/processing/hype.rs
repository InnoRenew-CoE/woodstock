use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use futures::future::join_all;
use once_cell::sync::Lazy;
use regex::RegexBuilder;
use tokio::sync::Semaphore;

use crate::rag::comm::question::Question;
use crate::rag::comm::{ChatClient, ImageInput};
use crate::rag::models::chunks::{Chunk, HypeChunk};
use crate::rag::models::{ChunkedFile, ImageRef};

/// How many requests the hype step keeps in flight at the same time. One request is
/// either one chunk of text or one image inside a chunk.
const DEFAULT_PARALLEL_HYPE_REQUESTS: usize = 32;

static PARALLEL_HYPE_REQUESTS: Lazy<usize> = Lazy::new(|| {
    let parallel = env::var("PARALLEL_HYPE_REQUESTS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|parallel| *parallel > 0)
        .unwrap_or(DEFAULT_PARALLEL_HYPE_REQUESTS);
    println!("[HYPE] running up to {parallel} request(s) at the same time");
    parallel
});

/// Every request waits for one of these slots before it goes to the model. The worker
/// hypes several documents at once and they all share this pool, so a document that is
/// almost finished leaves its free slots to the next one.
static HYPE_SLOTS: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(*PARALLEL_HYPE_REQUESTS)));

static LIST_PATTERN: Lazy<regex::Regex> = Lazy::new(|| {
    RegexBuilder::new(r"^\s*[\-\*]|\s*\d+\.\s*|\s*[a-zA-Z]\)\s*|\s*\(\d+\)\s*|\s*\([ivxlcdm]+\)\s*")
        .case_insensitive(true)
        .build()
        .unwrap()
});

/// One question prompt sent to the chat model.
struct HypeTask {
    /// Index of the chunk the questions belong to.
    chunk: usize,
    /// Image the questions come from, `None` when they come from the chunk text.
    image: Option<ImageRef>,
    question: Question,
}

pub async fn hype(file: ChunkedFile<Chunk>, llm: &ChatClient) -> ChunkedFile<HypeChunk> {
    let tasks = hype_tasks(&file);
    let answers = request_questions(&tasks, llm).await;

    let mut hype_chunks: Vec<HypeChunk> = file.chunks.iter().map(HypeChunk::from).collect();
    for (task, questions) in tasks.iter().zip(answers) {
        hype_chunks[task.chunk].questions.extend(questions);
    }
    for (index, chunk) in hype_chunks.iter().enumerate() {
        if chunk.questions.is_empty() {
            eprintln!("[HYPE] chunk {index} got no questions");
        }
    }

    replace_chunks(file, hype_chunks)
}

/// Asks for every task of the document. They all queue at once and the shared pool
/// decides how many of them reach the model. Answers keep the position of their task,
/// so a chunk gets its text questions first and its image questions after them.
async fn request_questions(tasks: &[HypeTask], llm: &ChatClient) -> Vec<Vec<String>> {
    let answers = join_all(tasks.iter().map(|task| ask(llm, task))).await;

    answers
        .into_iter()
        .map(|answer| match answer {
            Ok(questions) => questions,
            Err(error) => {
                eprintln!("[HYPE] {error:#}");
                Vec::new()
            }
        })
        .collect()
}

/// Asks the model for the questions of one task, text or image.
async fn ask(llm: &ChatClient, task: &HypeTask) -> Result<Vec<String>> {
    let _slot = HYPE_SLOTS.acquire().await?;

    let response = match &task.image {
        None => llm
            .generate(task.question.clone())
            .await
            .with_context(|| format!("questions for chunk {}", task.chunk))?,
        Some(image) => llm
            .generate_with_image(task.question.clone(), image_input(image).await?)
            .await
            .with_context(|| format!("questions for image {}", image.route))?,
    };

    Ok(split_questions(&response.content))
}

async fn image_input(image: &ImageRef) -> Result<ImageInput> {
    let bytes = tokio::fs::read(&image.path)
        .await
        .with_context(|| format!("read image for image hype {}", image.path.display()))?;
    let mime_type = mime_guess::from_path(&image.path).first_or_octet_stream().essence_str().to_string();

    Ok(ImageInput { bytes, mime_type })
}

fn hype_tasks(file: &ChunkedFile<Chunk>) -> Vec<HypeTask> {
    let mut tasks = Vec::new();

    // The chunk text goes first, then every image of that chunk, so the questions of a
    // chunk come out in the same order they always did.
    for (index, chunk) in file.chunks.iter().enumerate() {
        tasks.push(HypeTask {
            chunk: index,
            image: None,
            question: chunk_question(&chunk.text),
        });
    }
    for (index, chunk) in file.chunks.iter().enumerate() {
        for image in &chunk.images {
            tasks.push(HypeTask {
                chunk: index,
                image: Some(image.clone()),
                question: image_question(image, &chunk.text),
            });
        }
    }

    tasks
}

fn chunk_question(text: &str) -> Question {
    let question = "/no_think You will be given a passage from a document.\n Your task is to analyze the context text (passage) and \
        generate essential questions that, when answered, capture the main points and core meaning of the text. \
        The questions should be exhaustive and understandable without context. When possible, named entities should be referenced by their full name. \
        Dont't refer to the text as 'the text' or 'the passage' and don't refer to outside material like 'Figure 2'. \
        However add questions that are diverse in topic. \
        It is extremely important that you only answer with questions and each question should be written in its own line (separated by newline) with no prefix.\
        And finally the answer to each question has to be found in the passage."
        .to_string();
    let system_prompt = "You are an agent specialized to only answer in list of questions.";

    Question::from(question)
        .set_system_prompt(system_prompt)
        .set_context(vec![format!("\nCONTEXT PASSAGE:\n{text}")])
}

fn image_question(image: &ImageRef, chunk_text: &str) -> Question {
    let prompt = "/no_think Generate retrieval questions that can be answered from the image and its surrounding document context. \
         Only answer with questions, one per line, with no numbering or prefixes.";

    Question::from(prompt)
        .set_system_prompt("You generate concise search questions from document images.")
        .set_context(vec![format!(
            "IMAGE ROUTE: {}\nALT TEXT: {}\nDOCUMENT CHUNK CONTEXT:\n{chunk_text}",
            image.route,
            image.alt_text.clone().unwrap_or_default()
        )])
}

fn split_questions(answer: &str) -> Vec<String> {
    answer
        .split('\n')
        .map(|line| LIST_PATTERN.replace(line, "").trim().to_string())
        .filter(|question| !question.is_empty())
        .collect()
}

fn replace_chunks(file: ChunkedFile<Chunk>, hype_chunks: Vec<HypeChunk>) -> ChunkedFile<HypeChunk> {
    let ChunkedFile {
        file_type,
        internal_id,
        tags,
        original_file_description,
        syntetic_file_description,
        images,
        ..
    } = file;

    ChunkedFile {
        file_type,
        chunks: hype_chunks,
        internal_id,
        tags,
        original_file_description,
        syntetic_file_description,
        images,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rag::RagProcessableFileType;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::path::Path;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    const CAP: usize = 4;
    const BODY: &str = r#"{"choices":[{"message":{"content":"- First question?\n- Second question?"}}]}"#;

    #[test]
    fn shares_the_parallel_hype_cap_between_documents() {
        std::env::set_var("PARALLEL_HYPE_REQUESTS", CAP.to_string());
        std::env::set_var("CHAT_PROVIDER", "openai-compatible");

        let server = FakeChatServer::start();
        std::env::set_var("OPENAI_COMPATIBLE_BASE_URL", server.base_url());
        let llm = ChatClient::default();

        let image_path = std::env::temp_dir().join(format!("woodstock-hype-{}.png", std::process::id()));
        std::fs::write(&image_path, b"not really a png").unwrap();

        let first = chunked_file(&image_path, "first");
        let second = chunked_file(&image_path, "second");
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (first, second) = runtime.block_on(async { tokio::join!(hype(first, &llm), hype(second, &llm)) });

        let peak = server.peak_requests();
        assert!(peak <= CAP, "{peak} requests ran at the same time, the cap is {CAP}");
        assert!(peak > 1, "requests never overlapped, peak was {peak}");
        assert!(
            server.documents_overlapped(),
            "the second document never had a request in flight while the first one was still going"
        );
        assert_eq!(
            server.total_requests(),
            12,
            "two documents of 3 chunk questions and 3 image questions each"
        );

        for hyped in [first, second] {
            let document = hyped.internal_id.clone();
            assert_eq!(hyped.chunks.len(), 3);
            for (index, chunk) in hyped.chunks.iter().enumerate() {
                assert_eq!(
                    chunk.questions,
                    vec!["First question?", "Second question?", "First question?", "Second question?"],
                    "chunk {index} of {document} should hold its text questions and then its image questions"
                );
            }
        }
    }

    /// A chat completion server that answers after a short pause. It records how many
    /// requests it handled at the same time and whether two documents ever had a request
    /// in flight together.
    struct FakeChatServer {
        base_url: String,
        in_flight: Arc<AtomicUsize>,
        peak: Arc<AtomicUsize>,
        total: Arc<AtomicUsize>,
        first_in_flight: Arc<AtomicUsize>,
        second_in_flight: Arc<AtomicUsize>,
        overlap: Arc<AtomicBool>,
    }

    impl FakeChatServer {
        fn start() -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let base_url = format!("http://{}/v1", listener.local_addr().unwrap());
            let in_flight = Arc::new(AtomicUsize::new(0));
            let peak = Arc::new(AtomicUsize::new(0));
            let total = Arc::new(AtomicUsize::new(0));
            let first_in_flight = Arc::new(AtomicUsize::new(0));
            let second_in_flight = Arc::new(AtomicUsize::new(0));
            let overlap = Arc::new(AtomicBool::new(false));

            let handled_in_flight = Arc::clone(&in_flight);
            let handled_peak = Arc::clone(&peak);
            let handled_total = Arc::clone(&total);
            let handled_first = Arc::clone(&first_in_flight);
            let handled_second = Arc::clone(&second_in_flight);
            let handled_overlap = Arc::clone(&overlap);
            std::thread::spawn(move || {
                for stream in listener.incoming() {
                    let Ok(mut stream) = stream else { continue };
                    let in_flight = Arc::clone(&handled_in_flight);
                    let peak = Arc::clone(&handled_peak);
                    let total = Arc::clone(&handled_total);
                    let first_in_flight = Arc::clone(&handled_first);
                    let second_in_flight = Arc::clone(&handled_second);
                    let overlap = Arc::clone(&handled_overlap);
                    std::thread::spawn(move || {
                        let request = read_request(&mut stream);
                        let is_first = request.contains("first chunk") || request.contains("/rag/images/first/");
                        let (mine, other) = if is_first {
                            (&first_in_flight, &second_in_flight)
                        } else {
                            (&second_in_flight, &first_in_flight)
                        };

                        let open = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                        peak.fetch_max(open, Ordering::SeqCst);
                        total.fetch_add(1, Ordering::SeqCst);
                        mine.fetch_add(1, Ordering::SeqCst);
                        if other.load(Ordering::SeqCst) > 0 {
                            overlap.store(true, Ordering::SeqCst);
                        }

                        std::thread::sleep(Duration::from_millis(50));
                        mine.fetch_sub(1, Ordering::SeqCst);
                        in_flight.fetch_sub(1, Ordering::SeqCst);
                        let _ = stream.write_all(response().as_bytes());
                    });
                }
            });

            Self {
                base_url,
                in_flight,
                peak,
                total,
                first_in_flight,
                second_in_flight,
                overlap,
            }
        }

        fn base_url(&self) -> String {
            self.base_url.clone()
        }

        fn peak_requests(&self) -> usize {
            self.peak.load(Ordering::SeqCst)
        }

        fn total_requests(&self) -> usize {
            self.total.load(Ordering::SeqCst)
        }

        fn documents_overlapped(&self) -> bool {
            self.overlap.load(Ordering::SeqCst)
        }
    }

    fn read_request(stream: &mut TcpStream) -> String {
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 4096];
        while !buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            match stream.read(&mut chunk) {
                Ok(0) | Err(_) => return String::new(),
                Ok(read) => buffer.extend_from_slice(&chunk[..read]),
            }
        }

        let head_length = buffer.windows(4).position(|window| window == b"\r\n\r\n").unwrap_or(buffer.len()) + 4;
        let head = String::from_utf8_lossy(&buffer[..head_length]).to_string();
        let length = head
            .lines()
            .find_map(|line| {
                let line = line.to_ascii_lowercase();
                line.strip_prefix("content-length: ")
                    .map(|value| value.trim().parse::<usize>().unwrap_or(0))
            })
            .unwrap_or(0);
        let mut missing = length.saturating_sub(buffer.len() - head_length);
        while missing > 0 {
            match stream.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    buffer.extend_from_slice(&chunk[..read]);
                    missing = missing.saturating_sub(read);
                }
            }
        }

        String::from_utf8_lossy(&buffer).into_owned()
    }

    fn response() -> String {
        format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{BODY}",
            BODY.len()
        )
    }

    fn chunked_file(image_path: &Path, document_id: &str) -> ChunkedFile<Chunk> {
        let chunks = (0..3)
            .map(|seq_num| Chunk {
                seq_num,
                text: format!("{document_id} chunk {seq_num} text"),
                embedding_vector: None,
                images: vec![ImageRef {
                    id: format!("{document_id}-image-{seq_num}"),
                    document_id: document_id.to_owned(),
                    file_name: "image.png".to_owned(),
                    route: format!("/rag/images/{document_id}/image-{seq_num}.png"),
                    alt_text: None,
                    path: image_path.to_path_buf(),
                }],
            })
            .collect();

        ChunkedFile {
            file_type: RagProcessableFileType::Pdf,
            chunks,
            internal_id: document_id.to_owned(),
            original_file_description: None,
            syntetic_file_description: None,
            tags: None,
            images: vec![],
        }
    }
}
