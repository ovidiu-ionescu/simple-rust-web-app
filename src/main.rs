mod render;
mod server;
mod data;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    server::start_server().await?;
    Ok(())
}
