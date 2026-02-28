use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum ExperienceCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),

    #[error(transparent)]
    PrefixResolve(#[from] PrefixError),

    #[error("at least one of --description or --sensation must be provided")]
    NoUpdateProvided,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ExperienceOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateExperienceOutcomes),
    #[outcome(transparent)]
    List(#[from] ListExperiencesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowExperienceOutcomes),
    #[outcome(transparent)]
    Update(#[from] UpdateExperienceOutcomes),
}

#[derive(Clone, Args)]
pub struct ExperienceOps {
    #[command(subcommand)]
    pub command: ExperienceCommands,
}

impl ExperienceOps {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ExperienceOutcomes>, ExperienceCommandError> {
        Ok(match &self.command {
            ExperienceCommands::Create(cmd) => cmd.run(context).await?.map_into(),
            ExperienceCommands::List(cmd) => cmd.run(context).await?.map_into(),
            ExperienceCommands::Show(cmd) => cmd.run(context).await?.map_into(),
            ExperienceCommands::Update(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub enum ExperienceCommands {
    /// Create a new experience (descriptive edge connecting cognitive records).
    Create(CreateExperience),
    /// List experiences, optionally filtered by agent or kind.
    List(ListExperiences),
    /// Show an experience's details by ID.
    Show(ShowExperience),
    /// Update an experience's description or sensation.
    Update(UpdateExperience),
}
