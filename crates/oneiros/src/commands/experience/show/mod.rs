mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowExperienceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowExperience {
    /// The experience ID to display.
    id: ExperienceId,
}

impl ShowExperience {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowExperienceOutcomes>, ExperienceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_experience(&context.ticket_token()?, &self.id)
            .await?;
        outcomes.emit(ShowExperienceOutcomes::ExperienceDetails(info));

        Ok(outcomes)
    }
}
