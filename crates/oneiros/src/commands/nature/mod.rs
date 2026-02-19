mod outcomes;

mod error;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::NatureCommandError;
pub(crate) use list::{ListNatures, ListNaturesOutcomes};
pub(crate) use outcomes::NatureOutcomes;
pub(crate) use remove::{RemoveNature, RemoveNatureOutcomes};
pub(crate) use set::{SetNature, SetNatureOutcomes};
pub(crate) use show::{ShowNature, ShowNatureOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct NatureOps {
    #[command(subcommand)]
    pub command: NatureCommands,
}

impl NatureOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<NatureOutcomes>, NatureCommandError> {
        Ok(match &self.command {
            NatureCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            NatureCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            NatureCommands::List(cmd) => cmd.run(context).await?.map_into(),
            NatureCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum NatureCommands {
    /// Create or update a nature.
    Set(SetNature),
    /// Remove a nature.
    Remove(RemoveNature),
    /// List all natures.
    List(ListNatures),
    /// Show a nature's details.
    Show(ShowNature),
}
