use crate::rag::loading::loaded_data::LoadedFile;

use super::{chunk::Chunk, chunked_file::ChunkedFile, ChunkOverlap, ChunkSize,};



pub fn simple_word_chunking(file: LoadedFile, chunk_size: &ChunkSize, overlap: &ChunkOverlap) -> ChunkedFile {
    let chunk_size = *chunk_size as usize;
    let overlap = *overlap as usize;

    let words: Vec<&str> = file.content.split_whitespace().collect();

    let mut chunks = Vec::new();
    let mut start_index = 0;
    let mut chunk_id = 0;

    while start_index < words.len() {
        let end_index = std::cmp::min(start_index + chunk_size, words.len());
        let chunk_words = &words[start_index..end_index];
        let text = chunk_words.join(" ");

        chunks.push(Chunk {
            id: chunk_id,
            text,
        });
        chunk_id += 1;

        if end_index >= words.len() {
            break;
        }

        let step = chunk_size.saturating_sub(overlap);
        start_index += step;
    }

    (file, chunks).into()
}