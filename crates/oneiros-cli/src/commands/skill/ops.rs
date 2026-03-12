use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum SkillCommandError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(
        "No project detected. Use --project from a project directory, or omit it to install globally."
    )]
    NoProject,

    #[error("Could not determine home directory.")]
    NoHomeDir,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SkillOutcomes {
    #[outcome(transparent)]
    Install(#[from] InstallSkillOutcomes),
}

/// Manage the oneiros skill plugin.
#[derive(Clone, Args)]
pub struct SkillOps {
    #[command(subcommand)]
    pub command: SkillCommands,
}

impl SkillOps {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SkillOutcomes>, SkillCommandError> {
        Ok(match &self.command {
            SkillCommands::Install(install) => install.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum SkillCommands {
    /// Install the oneiros skill for Claude Code.
    Install(InstallSkill),
}
