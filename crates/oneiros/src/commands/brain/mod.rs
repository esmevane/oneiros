mod error;
mod outcomes;
mod replay;

pub(crate) use error::BrainCommandError;
pub(crate) use outcomes::BrainOutcomes;
pub(crate) use replay::{ReplayBrain, ReplayBrainOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct BrainOps {
    #[command(subcommand)]
    pub command: BrainCommands,
}

impl BrainOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<BrainOutcomes>, BrainCommandError> {
        Ok(match &self.command {
            BrainCommands::Replay(replay) => replay.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum BrainCommands {
    /// Replay the brain's event log onto a fresh database with content-addressed IDs.
    Replay(ReplayBrain),
}
