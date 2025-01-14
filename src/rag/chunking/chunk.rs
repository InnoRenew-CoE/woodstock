use crate::rag::comm::embedding::Embeddable;

#[derive(Debug)]
pub struct Chunk {
    pub id: i32,
    pub text: String,
}

impl Embeddable for Chunk {

}