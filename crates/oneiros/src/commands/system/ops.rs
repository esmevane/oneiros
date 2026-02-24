use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SystemCommandError {
    #[error("Error during initialization: {0}")]
    Init(#[from] InitSystemError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SystemOutcomes {
    #[outcome(transparent)]
    InitOutcome(#[from] InitSystemOutcomes),
}

#[derive(Clone, Args)]
pub struct SystemOps {
    #[command(subcommand)]
    pub command: SystemCommand,
}

impl SystemOps {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SystemOutcomes>, SystemCommandError> {
        Ok(match &self.command {
            SystemCommand::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum SystemCommand {
    /// Initializes a local oneiros host.
    Init(Init),
}
