#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    oneiros_engine::Engine::run().await?;

    Ok(())
}
