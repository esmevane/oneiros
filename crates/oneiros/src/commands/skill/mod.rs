mod error;
mod install;
mod outcomes;

pub(crate) use error::SkillCommandError;
pub(crate) use install::{InstallSkill, InstallSkillOutcomes};
pub(crate) use outcomes::SkillOutcomes;

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

use crate::*;

/// Manage the oneiros skill plugin.
#[derive(Clone, Args)]
pub(crate) struct SkillOps {
    #[command(subcommand)]
    pub command: SkillCommands,
}

impl SkillOps {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SkillOutcomes>, SkillCommandError> {
        Ok(match &self.command {
            SkillCommands::Install(install) => install.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum SkillCommands {
    /// Install the oneiros skill for Claude Code.
    Install(InstallSkill),
}
