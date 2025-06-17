use simple::simple_word_chunking;

use super::{
    loading::loaded_data::LoadedFile,
    models::{chunks::Chunk, ChunkedFile},
};

mod dedup_embeddings;
mod embedd_file;
mod hype;
mod prepare;
mod prompt;
mod simple;
mod summarize;

pub use dedup_embeddings::dedup;
pub use hype::hype;
pub use prepare::prepare_for_upload;
pub use prompt::prompt;

type ChunkSize = i32;
type ChunkOverlap = i32;

pub enum ChunkingStrategy {
    Word(ChunkSize, ChunkOverlap),
}

pub fn chunk(file: LoadedFile, strategy: ChunkingStrategy) -> ChunkedFile<Chunk> {
    match &strategy {
        ChunkingStrategy::Word(size, overlap) => simple_word_chunking(file, size, overlap),
    }
}
