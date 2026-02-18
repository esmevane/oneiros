mod error;
mod outcomes;
mod status;

pub(crate) use error::ActivityError;
pub(crate) use outcomes::ActivityOutcomes;
pub(crate) use status::ActivityStatus;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

use crate::*;

/// Monitor agent activity across cognitive domains.
#[derive(Clone, Args)]
pub(crate) struct ActivityOps {
    #[command(subcommand)]
    pub command: ActivityCommands,
}

impl ActivityOps {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ActivityOutcomes>, ActivityError> {
        Ok(match &self.command {
            ActivityCommands::Status(cmd) => cmd.run(context).await?,
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum ActivityCommands {
    /// Show activity freshness for all agents across all cognitive domains.
    Status(ActivityStatus),
}
