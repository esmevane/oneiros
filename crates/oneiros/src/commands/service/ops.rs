use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ServiceCommandError {
    #[error("Service error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] oneiros_db::DatabaseError),

    #[error("System not initialized. Run `oneiros system init` first.")]
    NotInitialized,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ServiceOutcomes {
    #[outcome(transparent)]
    Install(#[from] InstallServiceOutcomes),
    #[outcome(transparent)]
    Uninstall(#[from] UninstallServiceOutcomes),
    #[outcome(transparent)]
    Start(#[from] StartServiceOutcomes),
    #[outcome(transparent)]
    Stop(#[from] StopServiceOutcomes),
    #[outcome(transparent)]
    Run(#[from] RunServiceOutcomes),
    #[outcome(transparent)]
    Status(#[from] ServiceStatusOutcomes),
}

#[derive(Clone, Args)]
pub struct ServiceOps {
    #[command(subcommand)]
    pub command: ServiceCommands,
}

impl ServiceOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ServiceOutcomes>, ServiceCommandError> {
        Ok(match &self.command {
            ServiceCommands::Install(install) => install.run(context).await?.map_into(),
            ServiceCommands::Uninstall(uninstall) => uninstall.run(context).await?.map_into(),
            ServiceCommands::Start(start) => start.run(context).await?.map_into(),
            ServiceCommands::Stop(stop) => stop.run(context).await?.map_into(),
            ServiceCommands::Run(run) => run.run(context).await?.map_into(),
            ServiceCommands::Status(status) => status.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ServiceCommands {
    /// Install oneiros as a managed user service.
    Install(InstallService),
    /// Remove the managed oneiros service.
    Uninstall(UninstallService),
    /// Start the managed oneiros service.
    Start(StartService),
    /// Stop the managed oneiros service.
    Stop(StopService),
    /// Start oneiros in the foreground.
    Run(RunService),
    /// Check if oneiros is running.
    Status(Status),
}
