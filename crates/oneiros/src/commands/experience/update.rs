use clap::Args;
use oneiros_client::Client;
use oneiros_model::{ExperienceId, RefToken};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceUpdatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UpdateExperienceOutcomes {
    #[outcome(message("Experience updated: {}", .0.ref_token), prompt("{}", .0.gauge))]
    ExperienceUpdated(ExperienceUpdatedResult),
}

#[derive(Clone, Args)]
pub struct UpdateExperience {
    /// The experience ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,

    /// The new description for the experience.
    description: Description,
}

impl UpdateExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<UpdateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ExperienceId(id),
            None => {
                let all = client.list_experiences(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|e| e.id.0).collect();
                ExperienceId(self.id.resolve(&ids)?)
            }
        };

        let experience = client
            .update_experience_description(
                &token,
                &id,
                UpdateExperienceDescriptionRequest {
                    description: self.description.clone(),
                },
            )
            .await?;

        let agents = client.list_agents(&token).await?;
        let gauge_str = agents
            .iter()
            .find(|agent| agent.id == experience.agent_id)
            .map(|agent| agent.name.clone());

        let gauge_str = if let Some(agent_name) = gauge_str {
            let all = client
                .list_experiences(&token, Some(&agent_name), None)
                .await?;
            crate::gauge::experience_gauge(&agent_name, &all)
        } else {
            String::new()
        };

        let ref_token = experience.ref_token();

        outcomes.emit(UpdateExperienceOutcomes::ExperienceUpdated(
            ExperienceUpdatedResult {
                id: experience.id,
                ref_token,
                gauge: gauge_str,
            },
        ));

        Ok(outcomes)
    }
}
