use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum StorageCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum StorageOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetStorageOutcomes),
    #[outcome(transparent)]
    Get(#[from] GetStorageOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveStorageOutcomes),
    #[outcome(transparent)]
    List(#[from] ListStorageOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowStorageOutcomes),
}

#[derive(Clone, Args)]
pub struct StorageOps {
    #[command(subcommand)]
    pub command: StorageCommands,
}

impl StorageOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<StorageOutcomes>, StorageCommandError> {
        Ok(match &self.command {
            StorageCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            StorageCommands::Get(cmd) => cmd.run(context).await?.map_into(),
            StorageCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            StorageCommands::List(cmd) => cmd.run(context).await?.map_into(),
            StorageCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum StorageCommands {
    /// Store a file under a key.
    Set(SetStorage),
    /// Download stored content to a file.
    Get(GetStorage),
    /// Remove a storage entry.
    Remove(RemoveStorage),
    /// List all storage entries.
    List(ListStorage),
    /// Show metadata for a storage entry.
    Show(ShowStorage),
}
