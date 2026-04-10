use std::io::Write;

use anstream::stdout;
use oneiros_engine::{Engine, OutputMode, Rendered, Responses};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let (engine, cli) = Engine::from_cli()?;

    // Apply color choice before any output.
    engine.config().color.apply_global();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let result: Rendered<Responses> = engine.execute(&cli).await?;

    let as_json = serde_json::to_string(result.response())?;

    let mut out = stdout().lock();

    match (
        &engine.config().output,
        result.has_prompt(),
        result.has_text(),
    ) {
        (OutputMode::Prompt, true, _) => write!(out, "{}", result.prompt())?,
        (OutputMode::Text, _, true) => write!(out, "{}", result.text())?,
        (OutputMode::Json, _, _) | (_, false, _) | (_, _, false) => writeln!(out, "{as_json}")?,
    }

    Ok(())
}
