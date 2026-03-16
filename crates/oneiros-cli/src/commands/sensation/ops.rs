use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SensationCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SensationOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetSensationOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveSensationOutcomes),
    #[outcome(transparent)]
    List(#[from] ListSensationsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowSensationOutcomes),
}

#[derive(Clone, Args)]
pub struct SensationOps {
    #[command(subcommand)]
    pub command: SensationCommands,
}

impl SensationOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<(Outcomes<SensationOutcomes>, Vec<PressureSummary>), SensationCommandError> {
        Ok(match &self.command {
            SensationCommands::Set(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            SensationCommands::Remove(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            SensationCommands::List(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
            SensationCommands::Show(cmd) => {
                let (o, s) = cmd.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum SensationCommands {
    /// Create or update a sensation.
    Set(SetSensation),
    /// Remove a sensation.
    Remove(RemoveSensation),
    /// List all sensations.
    List(ListSensations),
    /// Show a sensation's details.
    Show(ShowSensation),
}
