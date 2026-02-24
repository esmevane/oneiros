use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum CognitionCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(transparent)]
    PrefixResolve(#[from] PrefixError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum CognitionOutcomes {
    #[outcome(transparent)]
    Add(#[from] AddCognitionOutcomes),
    #[outcome(transparent)]
    List(#[from] ListCognitionsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowCognitionOutcomes),
}

#[derive(Clone, Args)]
pub struct CognitionOps {
    #[command(subcommand)]
    pub command: CognitionCommands,
}

impl CognitionOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<CognitionOutcomes>, CognitionCommandError> {
        Ok(match &self.command {
            CognitionCommands::Add(cmd) => cmd.run(context).await?.map_into(),
            CognitionCommands::List(cmd) => cmd.run(context).await?.map_into(),
            CognitionCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum CognitionCommands {
    /// Add a new cognition (thought) for an agent.
    Add(AddCognition),
    /// List cognitions, optionally filtered by agent or texture.
    List(ListCognitions),
    /// Show a cognition's details by ID.
    Show(ShowCognition),
}
