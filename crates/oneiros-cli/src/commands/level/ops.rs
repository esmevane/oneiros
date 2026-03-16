use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum LevelCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum LevelOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetLevelOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveLevelOutcomes),
    #[outcome(transparent)]
    List(#[from] ListLevelsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowLevelOutcomes),
}

#[derive(Clone, Args)]
pub struct LevelOps {
    #[command(subcommand)]
    pub command: LevelCommands,
}

impl LevelOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<(Outcomes<LevelOutcomes>, Vec<PressureSummary>), LevelCommandError> {
        Ok(match &self.command {
            LevelCommands::Set(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            LevelCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            LevelCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            LevelCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum LevelCommands {
    /// Create or update a level.
    Set(SetLevel),
    /// Remove a level.
    Remove(RemoveLevel),
    /// List all levels.
    List(ListLevels),
    /// Show a level's details.
    Show(ShowLevel),
}
