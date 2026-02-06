mod cli;
mod commands;
mod context;
mod error;
mod events;
mod logging;
mod models;
mod projections;
mod values;

use clap::Parser;

pub(crate) use cli::*;
pub(crate) use commands::*;
pub(crate) use context::*;
pub(crate) use events::*;
pub(crate) use logging::*;
pub(crate) use models::*;
pub(crate) use values::*;

pub use error::*;

pub async fn run() -> Result<(), Error> {
    logging::init()?;

    let cli = Cli::parse();
    cli.run().await?;

    Ok(())
}
