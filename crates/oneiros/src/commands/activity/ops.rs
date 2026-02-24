use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ActivityError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

/// Monitor agent activity across cognitive domains.
#[derive(Clone, Args)]
pub struct ActivityOps {
    #[command(subcommand)]
    pub command: ActivityCommands,
}

impl ActivityOps {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ActivityOutcomes>, ActivityError> {
        Ok(match &self.command {
            ActivityCommands::Status(cmd) => cmd.run(context).await?,
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ActivityCommands {
    /// Show activity freshness for all agents across all cognitive domains.
    Status(ActivityStatus),
}
