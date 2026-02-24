use clap::{Args, Subcommand};
use oneiros_client::Client;
use oneiros_model::{Id, RecordKind, Token};
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

    #[error("Invalid ref format: {0}")]
    InvalidRefFormat(String),
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
    RefAdd(#[from] RefAddOutcomes),
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
            ExperienceCommands::Ref(cmd) => cmd.run(context).await?.map_into(),
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
    /// Manage experience references to cognitive records.
    #[command(subcommand)]
    Ref(RefCommands),
    /// Update an experience's description.
    Update(UpdateExperience),
}

#[derive(Clone, Subcommand)]
pub enum RefCommands {
    /// Add a reference to a cognitive record.
    Add(RefAdd),
}

impl RefCommands {
    pub async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ExperienceOutcomes>, ExperienceCommandError> {
        Ok(match self {
            RefCommands::Add(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

pub(super) async fn list_ids_for_kind(
    client: &Client,
    token: &Token,
    kind: &RecordKind,
) -> Result<Vec<Id>, ExperienceCommandError> {
    let ids = match kind {
        RecordKind::Cognition => {
            let all = client.list_cognitions(token, None, None).await?;
            all.iter().map(|c| c.id.0).collect()
        }
        RecordKind::Memory => {
            let all = client.list_memories(token, None, None).await?;
            all.iter().map(|m| m.id.0).collect()
        }
        RecordKind::Experience => {
            let all = client.list_experiences(token, None, None).await?;
            all.iter().map(|e| e.id.0).collect()
        }
        RecordKind::Storage => {
            vec![]
        }
    };

    Ok(ids)
}
