mod error;
mod init;
mod outcomes;

pub(crate) use error::*;
pub(crate) use init::*;
pub(crate) use outcomes::*;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SystemOps {
    #[command(subcommand)]
    pub command: SystemCommand,
}

impl SystemOps {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SystemOutcomes>, SystemCommandError> {
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
