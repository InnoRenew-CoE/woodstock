use std::{sync::Arc, time::Duration}; 
use crate::rag::{ 
    comm::{ 
        embedding::{Embeddable, EmbeddingVector}, 
        OllamaClient, 
    },
    models::ChunkedFile, 
}; 
use anyhow::{anyhow, Result}; 
use futures::stream::FuturesUnordered; 
use rand::{thread_rng, Rng}; 
use tokio::{sync::Semaphore, time::sleep}; 
use futures::StreamExt; 

const MAX_ATTEMPTS: usize = 4; 
const BASE_DELAY_MS: u64 = 1000; 
const MAX_CONCURRENCY: usize = 64; 

pub async fn embedd_file<T>( 
    mut file: ChunkedFile<T>, 
    ollama: &OllamaClient 
) -> Result<ChunkedFile<T>> 
where 
    T: Embeddable + Clone, 
{ 
    let sem = Arc::new(Semaphore::new(MAX_CONCURRENCY)); 
    let mut futs = FuturesUnordered::new(); 
    println!("Embedding chunks"); 
    for mut c in file.chunks.into_iter() { 
        let sem = sem.clone(); 
        futs.push(async move { 
            let _permit = sem 
                .acquire() 
                .await 
                .expect("semaphore"); 
            match embedd_questions(&mut c, ollama).await { 
                Ok(_) => Some(c), 
                Err(_) => None, 
            } 
        }); 
    } 
    
    let mut kept = Vec::new(); 
    while let Some(opt) = futs.next().await { 
        if let Some(c) = opt { 
            kept.push(c);
        } 
    } 
    
    println!("Embedded {} chunks", kept.len()); 
    file.chunks = kept; 
    Ok(file) 
} 

async fn embedd_questions<T>( 
    chunk: &mut T, 
    client: &OllamaClient, 
) -> Result<()> 
where 
    T: Embeddable + Clone, 
{ 
    for _ in 0..MAX_ATTEMPTS { 
        let req = chunk.clone().try_into_embed(); 
        match client.embed(req).await { 
            Ok(resp) => { 
                let vectors = resp 
                    .embeddings 
                    .into_iter() 
                    .map(EmbeddingVector) 
                    .collect::<Vec<_>>(); 
                chunk.set_embedding_vectors(vectors); 
                return Ok(()); 
            } 
            Err(err) => { 
                let jitter = jitter_ms(BASE_DELAY_MS as u64 / 2); 
                sleep(Duration::from_millis(BASE_DELAY_MS + jitter)).await; 
                println!("Error embedding: {:#?}", err) 
            } 
        } 
    } 
    Err(anyhow!("Could not embedd chunk")) 
} 

fn jitter_ms(max: u64) -> u64 { if max == 0 { 0 } else { thread_rng().gen_range(0..=max) } }