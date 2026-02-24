use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ProjectCommandError {
    #[error(transparent)]
    Context(#[from] ContextError),

    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Malformed JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("No project detected. Run this from within a project directory.")]
    NoProject,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ProjectOutcomes {
    #[outcome(transparent)]
    Init(#[from] InitProjectOutcomes),
    #[outcome(transparent)]
    Export(#[from] ExportProjectOutcomes),
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
            ProjectCommands::Export(export) => export.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ProjectCommands {
    /// Initialize a brain for the current project.
    Init(InitProject),
    /// Export a brain to the target directory (defaults to current directory)
    Export(ExportProject),
}
