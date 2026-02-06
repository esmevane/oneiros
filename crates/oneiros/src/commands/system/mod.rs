mod error;
mod init;
mod init_outcomes;

pub(crate) use error::*;
pub(crate) use init::*;
pub(crate) use init_outcomes::*;

use clap::{Args, Subcommand};

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
    ) -> Result<Vec<SystemOutcome>, SystemCommandError> {
        Ok(match &self.command {
            SystemCommand::Init(init) => init.run(context).await.map(to_system_outcome)?,
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum SystemCommand {
    /// Initializes a local oneiros host.
    Init(Init),
}

#[derive(Clone)]
pub(crate) enum SystemOutcome {
    InitOutcome(InitOutcomes),
}

impl Reportable for SystemOutcome {
    fn report(&self) {
        match self {
            Self::InitOutcome(init_outcome) => init_outcome.report(),
        }
    }
}

impl From<InitOutcomes> for SystemOutcome {
    fn from(value: InitOutcomes) -> Self {
        Self::InitOutcome(value)
    }
}

fn to_system_outcome<T>(output: Vec<T>) -> Vec<SystemOutcome>
where
    T: Into<SystemOutcome> + Clone,
{
    output.iter().cloned().map(Into::into).collect()
}
