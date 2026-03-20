use clap::Args;
use oneiros_model::Experience;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CreateExperienceOutcomes {
    #[outcome(message("Experience created: {}", .0.ref_token()))]
    ExperienceCreated(Experience),
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
    ) -> Result<
        (
            Outcomes<CreateExperienceOutcomes>,
            Vec<PressureSummary>,
            Option<RefToken>,
        ),
        ExperienceCommandError,
    > {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let create_response = client
            .create_experience(
                &token,
                CreateExperienceRequest {
                    agent: self.agent.clone(),
                    sensation: self.sensation.clone(),
                    description: self.description.clone(),
                },
            )
            .await?;
        let summaries = create_response.pressure_summaries();
        let ref_token = create_response.ref_token();
        let experience: Experience = create_response.data()?;

        outcomes.emit(CreateExperienceOutcomes::ExperienceCreated(experience));

        Ok((outcomes, summaries, ref_token))
    }
}
