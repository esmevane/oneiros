mod cli;
mod commands;
mod error;
mod gauge;
mod logging;
mod prefix_id;

use clap::Parser;

pub(crate) use logging::*;
pub(crate) use oneiros_context::{Context, ContextError};
pub(crate) use oneiros_model::*;
pub(crate) use oneiros_service::projections;
pub(crate) use prefix_id::*;

pub use cli::*;
pub use commands::*;
pub use error::*;

pub async fn run() -> Result<(), Error> {
    let preflight = Preflight::preflight_parse();

    logging::init(&preflight.log)?;

    let cli = Cli::parse();
    let result = cli.run().await?;

    cli.report(&result);

    Ok(())
}
