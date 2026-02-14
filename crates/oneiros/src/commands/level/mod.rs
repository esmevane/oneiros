mod outcomes;

mod error;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::LevelCommandError;
pub(crate) use list::{ListLevels, ListLevelsOutcomes};
pub(crate) use outcomes::LevelOutcomes;
pub(crate) use remove::{RemoveLevel, RemoveLevelOutcomes};
pub(crate) use set::{SetLevel, SetLevelOutcomes};
pub(crate) use show::{ShowLevel, ShowLevelOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct LevelOps {
    #[command(subcommand)]
    pub command: LevelCommands,
}

impl LevelOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<LevelOutcomes>, LevelCommandError> {
        Ok(match &self.command {
            LevelCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            LevelCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            LevelCommands::List(cmd) => cmd.run(context).await?.map_into(),
            LevelCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum LevelCommands {
    /// Create or update a level.
    Set(SetLevel),
    /// Remove a level.
    Remove(RemoveLevel),
    /// List all levels.
    List(ListLevels),
    /// Show a level's details.
    Show(ShowLevel),
}
