use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum UrgeCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum UrgeOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetUrgeOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveUrgeOutcomes),
    #[outcome(transparent)]
    List(#[from] ListUrgesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowUrgeOutcomes),
}

#[derive(Clone, Args)]
pub struct UrgeOps {
    #[command(subcommand)]
    pub command: UrgeCommands,
}

impl UrgeOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<UrgeOutcomes>, UrgeCommandError> {
        Ok(match &self.command {
            UrgeCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            UrgeCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            UrgeCommands::List(cmd) => cmd.run(context).await?.map_into(),
            UrgeCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum UrgeCommands {
    /// Create or update an urge.
    Set(SetUrge),
    /// Remove an urge.
    Remove(RemoveUrge),
    /// List all urges.
    List(ListUrges),
    /// Show an urge's details.
    Show(ShowUrge),
}
