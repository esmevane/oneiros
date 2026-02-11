#[tokio::main]
async fn main() -> Result<(), oneiros::Error> {
    let _ = oneiros::run().await?;

    Ok(())
}
