mod cli;
mod commands;
mod context;
mod database;
mod error;
mod logging;

use clap::Parser;

use cli::Cli;
use context::Context;

pub(crate) use commands::Doctor;
pub(crate) use database::Database;
pub(crate) use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn run() -> Result<()> {
    logging::init()?;

    let cli = Cli::parse();

    cli.run().await;

    Ok(())
}
