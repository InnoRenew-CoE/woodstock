use chunk::Chunk;
use chunked_file::ChunkedFile;
use simple::simple_word_chunking;

use super::loading::loaded_data::LoadedFile;

pub mod chunked_file;
pub mod chunk;
pub mod hype_chunk;
mod simple;

type ChunkSize = i32;
type ChunkOverlap = i32;

pub enum ChunkingStrategy {
    Word(ChunkSize, ChunkOverlap)
}

pub fn chunk(file: LoadedFile, strategy: ChunkingStrategy) -> ChunkedFile<Chunk> {
    match &strategy {
        ChunkingStrategy::Word(size, overlap) => simple_word_chunking(file, size, overlap),
    }
}