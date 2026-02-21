mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowExperienceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowExperience {
    /// The experience ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowExperience {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => ExperienceId(id),
            None => {
                let all = client.list_experiences(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|e| e.id.inner().clone()).collect();
                ExperienceId(self.id.resolve(&ids)?)
            }
        };

        let experience = client.get_experience(&token, &id).await?;

        outcomes.emit(ShowExperienceOutcomes::ExperienceDetails(
            outcomes::ExperienceDetail(experience),
        ));

        Ok(outcomes)
    }
}
