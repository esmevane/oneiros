mod outcomes;

use clap::Args;
use oneiros_client::{Client, UpdateExperienceDescriptionRequest};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::UpdateExperienceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct UpdateExperience {
    /// The experience ID to update.
    id: ExperienceId,

    /// The new description for the experience.
    description: Content,
}

impl UpdateExperience {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<UpdateExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let experience = client
            .update_experience_description(
                &context.ticket_token()?,
                &self.id,
                UpdateExperienceDescriptionRequest {
                    description: self.description.clone(),
                },
            )
            .await?;
        outcomes.emit(UpdateExperienceOutcomes::ExperienceUpdated(experience.id));

        Ok(outcomes)
    }
}
