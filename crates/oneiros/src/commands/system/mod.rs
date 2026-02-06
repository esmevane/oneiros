mod error;
mod init;
mod init_outcomes;

pub(crate) use error::*;
pub(crate) use init::*;
pub(crate) use init_outcomes::*;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct System {
    #[command(subcommand)]
    pub command: SystemCommand,
}

impl System {
    pub(crate) async fn run(
        &self,
        context: Option<Context>,
    ) -> Result<Outcomes<SystemOutcome>, SystemCommandError> {
        Ok(match &self.command {
            SystemCommand::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum SystemCommand {
    /// Initializes a local oneiros host.
    Init(Init),
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) enum SystemOutcome {
    InitOutcome(InitOutcomes),
}

impl From<InitOutcomes> for SystemOutcome {
    fn from(value: InitOutcomes) -> Self {
        Self::InitOutcome(value)
    }
}
