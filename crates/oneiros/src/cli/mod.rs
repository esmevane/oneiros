mod cli_outcomes;
mod commands;
mod error;
mod log_config;

pub(crate) use cli_outcomes::CliOutcome;
pub(crate) use commands::Command;
pub(crate) use error::*;
pub(crate) use log_config::LogConfig;

use clap::Parser;
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(Clone, Parser)]
#[command(version)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) log: LogConfig,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    pub(crate) async fn run(&self) -> Result<Outcomes<CliOutcome>, CliError> {
        let context = Context::discover().ok_or(CliPreconditionError::NoContext)?;

        Ok(match &self.command {
            Command::Doctor(doctor) => doctor.run(context).await?.map_into(),
            Command::System(system) => system.run(context).await?.map_into(),
            Command::Service(service) => service.run(context).await?.map_into(),
            Command::Project(project) => project.run(context).await?.map_into(),
        })
    }
}
