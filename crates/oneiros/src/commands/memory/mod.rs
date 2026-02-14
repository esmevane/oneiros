mod outcomes;

mod add;
mod error;
mod list;
mod show;

pub(crate) use add::{AddMemory, AddMemoryOutcomes};
pub(crate) use error::MemoryCommandError;
pub(crate) use list::{ListMemories, ListMemoriesOutcomes};
pub(crate) use outcomes::MemoryOutcomes;
pub(crate) use show::{ShowMemory, ShowMemoryOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct MemoryOps {
    #[command(subcommand)]
    pub command: MemoryCommands,
}

impl MemoryOps {
    pub(crate) async fn run(
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
pub(crate) enum MemoryCommands {
    /// Add a new memory for an agent.
    Add(AddMemory),
    /// List memories, optionally filtered by agent or level.
    List(ListMemories),
    /// Show a memory's details by ID.
    Show(ShowMemory),
}
