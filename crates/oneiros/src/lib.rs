mod cli;
mod commands;
mod context;
mod database;
mod error;
mod logging;

use clap::Parser;

use cli::{Cli, Full, Preflight};
use context::Context;

pub(crate) use commands::Doctor;
pub(crate) use database::Database;
pub(crate) use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn run() -> Result<()> {
    let preflight = Preflight::preflight_parse();

    logging::init(preflight)?;

    let cli = Full::parse();

    cli.run().await;

    Ok(())
}
