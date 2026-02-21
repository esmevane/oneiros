mod outcomes;

mod create;
mod error;
mod list;
mod ref_add;
mod show;
mod update;

pub(crate) use create::{CreateExperience, CreateExperienceOutcomes};
pub(crate) use error::ExperienceCommandError;
pub(crate) use list::{ListExperiences, ListExperiencesOutcomes};
pub(crate) use outcomes::ExperienceOutcomes;
pub(crate) use ref_add::{RefAdd, RefAddOutcomes};
pub(crate) use show::{ShowExperience, ShowExperienceOutcomes};
pub(crate) use update::{UpdateExperience, UpdateExperienceOutcomes};

use clap::{Args, Subcommand};
use oneiros_client::Client;
use oneiros_model::{Id, RecordKind, Token};
use oneiros_outcomes::Outcomes;

pub(super) async fn list_ids_for_kind(
    client: &Client,
    token: &Token,
    kind: &RecordKind,
) -> Result<Vec<Id>, ExperienceCommandError> {
    let ids = match kind {
        RecordKind::Cognition => {
            let all = client.list_cognitions(token, None, None).await?;
            all.iter().map(|c| c.id.inner().clone()).collect()
        }
        RecordKind::Memory => {
            let all = client.list_memories(token, None, None).await?;
            all.iter().map(|m| m.id.inner().clone()).collect()
        }
        RecordKind::Experience => {
            let all = client.list_experiences(token, None, None).await?;
            all.iter().map(|e| e.id.inner().clone()).collect()
        }
        RecordKind::Storage => {
            vec![]
        }
    };

    Ok(ids)
}

#[derive(Clone, Args)]
pub(crate) struct ExperienceOps {
    #[command(subcommand)]
    pub command: ExperienceCommands,
}

impl ExperienceOps {
    pub(crate) async fn run(
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
pub(crate) enum ExperienceCommands {
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
pub(crate) enum RefCommands {
    /// Add a reference to a cognitive record.
    Add(RefAdd),
}

impl RefCommands {
    pub(crate) async fn run(
        &self,
        context: &crate::Context,
    ) -> Result<Outcomes<ExperienceOutcomes>, ExperienceCommandError> {
        Ok(match self {
            RefCommands::Add(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}
