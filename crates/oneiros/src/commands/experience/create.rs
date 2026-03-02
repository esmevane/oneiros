use clap::Args;
use oneiros_model::ExperienceId;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct ExperienceCreatedResult {
    pub id: ExperienceId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {}", .0.ref_token), prompt("{}", .0.gauge))]
    ExperienceCreated(ExperienceCreatedResult),
}

#[derive(Clone, Args)]
pub struct CreateExperience {
    /// The agent who is creating this experience.
    agent: AgentName,

    /// The sensation of experience being created.
    sensation: SensationName,

    /// A description of the experience.
    description: Description,
}

impl CreateExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<CreateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let experience = client
            .create_experience(
                &token,
                CreateExperienceRequest {
                    agent: self.agent.clone(),
                    sensation: self.sensation.clone(),
                    description: self.description.clone(),
                },
            )
            .await?;

        let all = client
            .list_experiences(&token, Some(&self.agent), None)
            .await?;
        let gauge = crate::gauge::experience_gauge(&self.agent, &all);

        let ref_token = experience.ref_token();

        outcomes.emit(CreateExperienceOutcomes::ExperienceCreated(
            ExperienceCreatedResult {
                id: experience.id,
                ref_token,
                gauge,
            },
        ));

        Ok(outcomes)
    }
}
