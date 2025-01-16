use anyhow::Result;

mod db;
mod rag;
mod server;
mod shared;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = dotenv::dotenv() {
        return Err(e.into());
    }

    server::start_server().await;
    Ok(())
}
