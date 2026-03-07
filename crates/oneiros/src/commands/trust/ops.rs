use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum TrustCommandError {
    #[error("Error during trust initialization: {0}")]
    Init(#[from] TrustInitError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum TrustOutcomes {
    #[outcome(transparent)]
    InitOutcome(#[from] TrustInitOutcomes),
}

#[derive(Clone, Args)]
pub struct TrustOps {
    #[command(subcommand)]
    pub command: TrustCommand,
}

impl TrustOps {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<TrustOutcomes>, TrustCommandError> {
        Ok(match &self.command {
            TrustCommand::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum TrustCommand {
    /// Set up TLS for this oneiros installation.
    Init(TrustInit),
}
