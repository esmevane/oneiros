mod outcomes;

mod error;
mod get;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::StorageCommandError;
pub(crate) use get::{GetStorage, GetStorageOutcomes};
pub(crate) use list::{ListStorage, ListStorageOutcomes};
pub(crate) use outcomes::StorageOutcomes;
pub(crate) use remove::{RemoveStorage, RemoveStorageOutcomes};
pub(crate) use set::{SetStorage, SetStorageOutcomes};
pub(crate) use show::{ShowStorage, ShowStorageOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct StorageOps {
    #[command(subcommand)]
    pub command: StorageCommands,
}

impl StorageOps {
    pub(crate) async fn run(
        &self,
        context: crate::Context,
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
pub(crate) enum StorageCommands {
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
