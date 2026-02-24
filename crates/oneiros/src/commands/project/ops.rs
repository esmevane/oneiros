use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ProjectCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error("IO error: {0}")]
    Io(std::io::Error),

    #[error("No project detected. Run this from within a project directory.")]
    NoProject,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ProjectOutcomes {
    #[outcome(transparent)]
    Init(#[from] InitProjectOutcomes),
}

#[derive(Clone, Args)]
pub struct ProjectOps {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

impl ProjectOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ProjectOutcomes>, ProjectCommandError> {
        Ok(match &self.command {
            ProjectCommands::Init(init) => init.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ProjectCommands {
    /// Initialize a brain for the current project.
    Init(InitProject),
}
