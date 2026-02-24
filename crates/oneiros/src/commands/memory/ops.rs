use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum MemoryCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(transparent)]
    PrefixResolve(#[from] PrefixError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum MemoryOutcomes {
    #[outcome(transparent)]
    Add(#[from] AddMemoryOutcomes),
    #[outcome(transparent)]
    List(#[from] ListMemoriesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowMemoryOutcomes),
}

#[derive(Clone, Args)]
pub struct MemoryOps {
    #[command(subcommand)]
    pub command: MemoryCommands,
}

impl MemoryOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<MemoryOutcomes>, MemoryCommandError> {
        Ok(match &self.command {
            MemoryCommands::Add(cmd) => cmd.run(context).await?.map_into(),
            MemoryCommands::List(cmd) => cmd.run(context).await?.map_into(),
            MemoryCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum MemoryCommands {
    /// Add a new memory for an agent.
    Add(AddMemory),
    /// List memories, optionally filtered by agent or level.
    List(ListMemories),
    /// Show a memory's details by ID.
    Show(ShowMemory),
}
