use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UpdateExperienceOutcomes {
    #[outcome(message("Experience updated: {}", .0.ref_token()), prompt("Experience updated: {}", .0.ref_token()))]
    ExperienceUpdated(Experience),
}

#[derive(Clone, Args)]
pub struct UpdateExperience {
    /// The experience ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,

    /// New description for the experience.
    #[arg(long)]
    description: Option<Description>,

    /// New sensation for the experience.
    #[arg(long)]
    sensation: Option<SensationName>,
}

impl UpdateExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<UpdateExperienceOutcomes>, Vec<PressureSummary>), ExperienceCommandError>
    {
        if self.description.is_none() && self.sensation.is_none() {
            return Err(ExperienceCommandError::NoUpdateProvided);
        }

        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ExperienceId(id),
            None => {
                let all: Vec<Experience> =
                    client.list_experiences(&token, None, None).await?.data()?;
                let ids: Vec<_> = all.iter().map(|e| e.id.0).collect();
                ExperienceId(self.id.resolve(&ids)?)
            }
        };

        let mut experience: Option<Experience> = None;
        let mut summaries: Vec<PressureSummary> = vec![];

        if let Some(description) = &self.description {
            let response = client
                .update_experience_description(
                    &token,
                    &id,
                    UpdateExperienceDescriptionRequest {
                        id,
                        description: description.clone(),
                    },
                )
                .await?;
            summaries = response.pressure_summaries();
            experience = Some(response.data()?);
        }

        if let Some(sensation) = &self.sensation {
            let response = client
                .update_experience_sensation(
                    &token,
                    &id,
                    UpdateExperienceSensationRequest {
                        id,
                        sensation: sensation.clone(),
                    },
                )
                .await?;
            summaries = response.pressure_summaries();
            experience = Some(response.data()?);
        }

        // At least one flag was provided (checked above), so experience is Some.
        let experience = experience.unwrap();

        outcomes.emit(UpdateExperienceOutcomes::ExperienceUpdated(experience));

        Ok((outcomes, summaries))
    }
}
