use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum NatureCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum NatureOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetNatureOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveNatureOutcomes),
    #[outcome(transparent)]
    List(#[from] ListNaturesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowNatureOutcomes),
}

#[derive(Clone, Args)]
pub struct NatureOps {
    #[command(subcommand)]
    pub command: NatureCommands,
}

impl NatureOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<(Outcomes<NatureOutcomes>, Vec<PressureSummary>), NatureCommandError> {
        Ok(match &self.command {
            NatureCommands::Set(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            NatureCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            NatureCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            NatureCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum NatureCommands {
    /// Create or update a nature.
    Set(SetNature),
    /// Remove a nature.
    Remove(RemoveNature),
    /// List all natures.
    List(ListNatures),
    /// Show a nature's details.
    Show(ShowNature),
}
