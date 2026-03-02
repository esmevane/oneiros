mod cli;
mod commands;
mod error;
mod gauge;
mod logging;
mod prefix_id;

use clap::Parser;

pub(crate) use cli::*;
pub(crate) use commands::*;
pub(crate) use logging::*;
pub(crate) use oneiros_context::{Context, ContextError};
pub(crate) use oneiros_model::*;
pub(crate) use oneiros_service::projections;
pub(crate) use prefix_id::*;

pub use error::*;

pub async fn run() -> Result<(), Error> {
    logging::init()?;

    let cli = Cli::parse();
    let outcomes = cli.run().await?;

    cli.report(&outcomes);

    Ok(())
}
