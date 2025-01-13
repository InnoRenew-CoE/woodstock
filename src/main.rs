use rag::comm::ollama::OllamaClient;

pub mod rag;



#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let client = OllamaClient::default();
    let res = client.generate("How do you code in rust?".into()).await;
    println!("{:#?}", res);
}
