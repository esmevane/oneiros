mod error;
mod init;
mod init_outcomes;

pub(crate) use error::ProjectCommandError;
pub(crate) use init::ProjectInit;
pub(crate) use init_outcomes::ProjectInitOutcomes;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct Project {
    #[command(subcommand)]
    pub command: ProjectSubcommand,
}

impl Project {
    pub(crate) async fn run(
        &self,
        context: crate::Context,
    ) -> Result<Outcomes<ProjectOutcome>, ProjectCommandError> {
        Ok(match &self.command {
            ProjectSubcommand::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum ProjectSubcommand {
    /// Initialize a brain for the current project.
    Init(ProjectInit),
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) enum ProjectOutcome {
    Init(ProjectInitOutcomes),
}

impl From<ProjectInitOutcomes> for ProjectOutcome {
    fn from(value: ProjectInitOutcomes) -> Self {
        Self::Init(value)
    }
}
