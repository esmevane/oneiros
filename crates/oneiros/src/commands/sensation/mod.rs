mod outcomes;

mod error;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::SensationCommandError;
pub(crate) use list::{ListSensations, ListSensationsOutcomes};
pub(crate) use outcomes::SensationOutcomes;
pub(crate) use remove::{RemoveSensation, RemoveSensationOutcomes};
pub(crate) use set::{SetSensation, SetSensationOutcomes};
pub(crate) use show::{ShowSensation, ShowSensationOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct SensationOps {
    #[command(subcommand)]
    pub command: SensationCommands,
}

impl SensationOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<SensationOutcomes>, SensationCommandError> {
        Ok(match &self.command {
            SensationCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            SensationCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            SensationCommands::List(cmd) => cmd.run(context).await?.map_into(),
            SensationCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum SensationCommands {
    /// Create or update a sensation.
    Set(SetSensation),
    /// Remove a sensation.
    Remove(RemoveSensation),
    /// List all sensations.
    List(ListSensations),
    /// Show a sensation's details.
    Show(ShowSensation),
}
