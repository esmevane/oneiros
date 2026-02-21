mod outcomes;

use clap::Args;
use oneiros_client::{Client, UpdateExperienceDescriptionRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::{ExperienceUpdatedResult, UpdateExperienceOutcomes};

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct UpdateExperience {
    /// The experience ID (full UUID or 8+ character prefix).
    id: PrefixId,

    /// The new description for the experience.
    description: Description,
}

impl UpdateExperience {
    pub(crate) async fn run(
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
            .find(|a| a.id == experience.agent_id)
            .map(|agent| agent.name.clone());

        let gauge_str = if let Some(agent_name) = gauge_str {
            let all = client
                .list_experiences(&token, Some(&agent_name), None)
                .await?;
            crate::gauge::experience_gauge(&agent_name, &all)
        } else {
            String::new()
        };

        outcomes.emit(UpdateExperienceOutcomes::ExperienceUpdated(
            ExperienceUpdatedResult {
                id: experience.id,
                gauge: gauge_str,
            },
        ));

        Ok(outcomes)
    }
}
