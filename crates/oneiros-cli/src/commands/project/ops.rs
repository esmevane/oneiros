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
    #[outcome(transparent)]
    Import(#[from] ImportProjectOutcomes),
    #[outcome(transparent)]
    Replay(#[from] ReplayProjectOutcomes),
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
    ) -> Result<(Outcomes<ProjectOutcomes>, Vec<PressureSummary>), ProjectCommandError> {
        Ok(match &self.command {
            ProjectCommands::Init(init) => {
                let (o, s) = init.run(context).await?;
                (o.map_into(), s)
            }
            ProjectCommands::Export(export) => {
                let (o, s) = export.run(context).await?;
                (o.map_into(), s)
            }
            ProjectCommands::Import(import) => {
                let (o, s) = import.run(context).await?;
                (o.map_into(), s)
            }
            ProjectCommands::Replay(replay) => {
                let (o, s) = replay.run(context).await?;
                (o.map_into(), s)
            }
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ProjectCommands {
    /// Initialize a brain for the current project.
    Init(InitProject),
    /// Export a brain to the target directory (defaults to current directory)
    Export(ExportProject),
    /// Import events from a jsonl export into the brain.
    Import(ImportProject),
    /// Replay all events through projections, rebuilding the read model.
    Replay(ReplayProject),
}
