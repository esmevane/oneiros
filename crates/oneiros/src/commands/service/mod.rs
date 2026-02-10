mod error;
mod run;
mod service_outcomes;
mod status;

pub(crate) use error::ServiceCommandError;
pub(crate) use run::Run;
pub(crate) use service_outcomes::{RunOutcomes, StatusOutcomes};
pub(crate) use status::Status;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct Service {
    #[command(subcommand)]
    pub command: ServiceSubcommand,
}

impl Service {
    pub(crate) async fn run(
        &self,
        context: crate::Context,
    ) -> Result<Outcomes<ServiceOutcome>, ServiceCommandError> {
        Ok(match &self.command {
            ServiceSubcommand::Run(run) => run.run(context).await?.map_into(),
            ServiceSubcommand::Status(status) => status.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum ServiceSubcommand {
    /// Start the oneiros service (foreground).
    Run(Run),
    /// Check if the oneiros service is running.
    Status(Status),
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) enum ServiceOutcome {
    Run(RunOutcomes),
    Status(StatusOutcomes),
}

impl From<RunOutcomes> for ServiceOutcome {
    fn from(value: RunOutcomes) -> Self {
        Self::Run(value)
    }
}

impl From<StatusOutcomes> for ServiceOutcome {
    fn from(value: StatusOutcomes) -> Self {
        Self::Status(value)
    }
}
