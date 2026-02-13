mod cli;
mod commands;
mod context;
mod error;
mod logging;

use clap::Parser;

pub(crate) use cli::*;
pub(crate) use commands::*;
pub(crate) use context::*;
pub(crate) use logging::*;
pub(crate) use oneiros_model::projections;
pub(crate) use oneiros_model::*;

pub use error::*;

pub async fn run() -> Result<(), Error> {
    logging::init()?;

    let cli = Cli::parse();
    let outcomes = cli.run().await?;

    cli.report(&outcomes);

    Ok(())
}
