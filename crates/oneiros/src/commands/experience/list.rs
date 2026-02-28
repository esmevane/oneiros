use clap::Args;
use oneiros_client::Client;
use oneiros_model::Experience;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ExperienceList(pub Vec<Experience>);

impl core::fmt::Display for ExperienceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|experience| format!("{experience}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListExperiencesOutcomes {
    #[outcome(message("No experiences found."))]
    NoExperiences,

    #[outcome(
        message("{0}"),
        prompt("Which threads are still growing? Extend with `oneiros connection create`.")
    )]
    Experiences(ExperienceList),
}

#[derive(Clone, Args)]
pub struct ListExperiences {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by sensation.
    #[arg(long)]
    sensation: Option<SensationName>,
}

impl ListExperiences {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListExperiencesOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let experiences = client
            .list_experiences(
                &context.ticket_token()?,
                self.agent.as_ref(),
                self.sensation.as_ref(),
            )
            .await?;

        if experiences.is_empty() {
            outcomes.emit(ListExperiencesOutcomes::NoExperiences);
        } else {
            outcomes.emit(ListExperiencesOutcomes::Experiences(ExperienceList(
                experiences,
            )));
        }

        Ok(outcomes)
    }
}
