use std::env;

use once_cell::sync::Lazy;
use qdrant_client::{qdrant::{PointStruct, SearchResponse, UpsertPointsBuilder}, Qdrant};
use tokio::sync::Mutex;
use anyhow::Result;

use crate::rag::models::chunks::EmbeddedChunk;

use super::embedding::EmbeddingVector;


/// Static global client for accessing the Qdrant database.
///
/// This variable initializes a Qdrant client connection that is used to interact with the Qdrant vector database.
/// It is lazily instantiated and locked via a mutex to ensure thread-safe access across asynchronous tasks.
///
/// # Panics
/// - Panics if the connection to the Qdrant database cannot be established, indicating a configuration or network issue.
static QDRANT_CLIENT: Lazy<Mutex<Qdrant>> = Lazy::new(|| {
    let qdrant_server = env::var("QDRANT_SERVER").expect("QDRANT_SERVER not defined");
    let client = match Qdrant::from_url(&qdrant_server).build() {
        Ok(c) => c,
        Err(e) => panic!("Can't establish Qdrant DB connection: {:#?}", e),
    };
    Mutex::new(client)
});

/// Performs a vector search in the Qdrant database using a given embedding tensor.
///
/// This function converts the provided tensor into a vector of `f32` values and uses it to query the Qdrant database.
/// It searches for points in the specified collection that are nearest to the input vector, returning results with payloads.
///
/// # Parameters
/// - `embedding`: The tensor representing an embedding that needs to be searched within the Qdrant vector space.
///
/// # Returns
/// Returns a `Result` containing the search response from Qdrant if successful. This response includes details of the
/// nearest points found in the vector space.
///
/// # Errors
/// - Returns an error if the tensor conversion fails or if the Qdrant search query encounters issues.
pub async fn vector_search(embedding: EmbeddingVector) -> Result<SearchResponse> {
    let client = QDRANT_CLIENT.lock().await;
    let search_result = client
        .search_points(embedding)
        .await?;
    Ok(search_result.into())
}

pub async fn insert_chunks_to_qdrant(embedded_chunks: Vec<EmbeddedChunk>) -> Result<()> {
    println!("Upserting to qdrant...");
    let client = QDRANT_CLIENT.lock().await;
    let qdrant_collection = env::var("QDRANT_COLLECTION").expect("QDRANT_COLLECTION not defined");


    let points: Vec<PointStruct> = embedded_chunks
        .into_iter()
        .map(|c| c.into())
        .collect();


    client
        .upsert_points(UpsertPointsBuilder::new(qdrant_collection, points))
        .await?;

    Ok(())
}