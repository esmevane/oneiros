use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    oneiros_engine::Engine::run().await
}
