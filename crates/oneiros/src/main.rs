#[tokio::main]
async fn main() -> Result<(), oneiros_cli::Error> {
    oneiros_cli::run().await
}
