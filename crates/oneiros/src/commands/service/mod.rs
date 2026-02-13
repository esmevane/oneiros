mod error;
mod install;
mod outcomes;
mod run;
mod start;
mod status;
mod stop;
mod uninstall;

pub(crate) use error::ServiceCommandError;
pub(crate) use install::{InstallService, InstallServiceOutcomes};
pub(crate) use outcomes::ServiceOutcomes;
pub(crate) use run::{RunService, RunServiceOutcomes};
pub(crate) use start::{StartService, StartServiceOutcomes};
pub(crate) use status::{ServiceStatusOutcomes, Status};
pub(crate) use stop::{StopService, StopServiceOutcomes};
pub(crate) use uninstall::{UninstallService, UninstallServiceOutcomes};

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
pub(crate) enum ServiceCommands {
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
