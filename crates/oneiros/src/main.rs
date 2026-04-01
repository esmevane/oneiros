use oneiros_engine::{Engine, OutputMode, Rendered, Responses};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let (engine, cli) = Engine::from_cli()?;
    let result: Rendered<Responses> = engine.execute(&cli).await?;

    let as_json = serde_json::to_string(result.response())?;

    match (
        &engine.config().output,
        result.has_prompt(),
        result.has_text(),
    ) {
        (OutputMode::Prompt, true, _) => print!("{}", result.prompt()),
        (OutputMode::Text, _, true) => print!("{}", result.text()),
        (OutputMode::Json, _, _) | (_, false, _) | (_, _, false) => println!("{as_json}"),
    }

    Ok(())
}
