mod commands;
mod error;
mod log_config;

pub(crate) use commands::Command;
pub(crate) use error::*;
pub(crate) use log_config::LogConfig;

use clap::Parser;

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
    pub(crate) async fn run(&self) -> Result<Vec<Outcome>, CliError> {
        let context = Context::discover();

        Ok(match &self.command {
            Command::Doctor(doctor) => doctor.run(context).await.map(to_outcome)?,
            Command::System(system) => system.run(context).await.map(to_outcome)?,
        })
    }
}

fn to_outcome<T>(output: Vec<T>) -> Vec<Outcome>
where
    T: Into<Outcome> + Clone,
{
    output.iter().cloned().map(Into::into).collect()
}
