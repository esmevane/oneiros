mod error;
mod outcomes;
mod run;
mod status;

pub(crate) use error::ServiceCommandError;
pub(crate) use outcomes::ServiceOutcomes;
pub(crate) use run::{RunService, RunServiceOutcomes};
pub(crate) use status::{ServiceStatusOutcomes, Status};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct ServiceOps {
    #[command(subcommand)]
    pub command: ServiceCommands,
}

impl ServiceOps {
    pub(crate) async fn run(
        &self,
        context: crate::Context,
    ) -> Result<Outcomes<ServiceOutcomes>, ServiceCommandError> {
        Ok(match &self.command {
            ServiceCommands::Run(run) => run.run(context).await?.map_into(),
            ServiceCommands::Status(status) => status.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum ServiceCommands {
    /// Start oneiros in the foreground.
    Run(RunService),
    /// Check if oneiros is running.
    Status(Status),
}
