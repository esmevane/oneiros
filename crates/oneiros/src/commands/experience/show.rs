use clap::Args;
use oneiros_model::{Experience, ExperienceId};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ExperienceDetail(pub Experience);

impl core::fmt::Display for ExperienceDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_detail())
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowExperienceOutcomes {
    #[outcome(
        message("{0}"),
        prompt(
            "Does this connect to anything new? Add with `oneiros connection create <nature> <from-ref> <to-ref>`."
        )
    )]
    ExperienceDetails(ExperienceDetail),
}

#[derive(Clone, Args)]
pub struct ShowExperience {
    /// The experience ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,
}

impl ShowExperience {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ExperienceId(id),
            None => {
                let all = client.list_experiences(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|e| e.id.0).collect();
                ExperienceId(self.id.resolve(&ids)?)
            }
        };

        let experience = client.get_experience(&token, &id).await?;

        outcomes.emit(ShowExperienceOutcomes::ExperienceDetails(ExperienceDetail(
            experience,
        )));

        Ok(outcomes)
    }
}
