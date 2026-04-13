#![allow(unused_imports)]

mod cli;
mod client;
mod config;
mod contexts;
mod domains;
mod engine;
mod error;
mod events;
mod http;
mod macros;
mod mcp;
mod projections;
mod protocol;
mod reducers;
mod skill;
mod support;
#[cfg(test)]
mod tests;
mod values;

pub(crate) use cli::*;
pub(crate) use client::*;
pub(crate) use config::*;
pub(crate) use contexts::*;
pub(crate) use domains::*;
pub(crate) use error::*;
pub(crate) use events::*;
pub(crate) use http::*;
pub(crate) use mcp::*;
pub(crate) use projections::*;
pub(crate) use reducers::*;

pub use engine::*;
pub use protocol::*;
pub use skill::*;
pub use support::*;
pub use values::*;

use macros::*;

pub async fn init() -> Result<(), Box<dyn core::error::Error>> {
    use std::io::Write;

    let (engine, cli) = Engine::from_cli()?;

    engine.config().color.apply_global();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let result: Rendered<Responses> = engine.execute(&cli).await?;

    let mut out = anstream::stdout().lock();

    match (
        &engine.config().output,
        result.has_prompt(),
        result.has_text(),
    ) {
        (OutputMode::Prompt, true, _) => write!(out, "{}", result.prompt())?,
        (OutputMode::Text, _, true) => write!(out, "{}", result.text())?,
        (OutputMode::Json, _, _) | (_, false, _) | (_, _, false) => writeln!(
            out,
            "{as_json}",
            as_json = serde_json::to_string(result.response())?
        )?,
    }

    Ok(())
}
