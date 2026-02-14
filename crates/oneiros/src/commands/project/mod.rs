mod error;
mod init;
mod outcomes;

pub(crate) use error::ProjectCommandError;
pub(crate) use init::{InitProject, InitProjectOutcomes};
pub(crate) use outcomes::ProjectOutcomes;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct ProjectOps {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

impl ProjectOps {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ProjectOutcomes>, ProjectCommandError> {
        Ok(match &self.command {
            ProjectCommands::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum ProjectCommands {
    /// Initialize a brain for the current project.
    Init(InitProject),
}
