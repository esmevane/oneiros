mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListExperiencesOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListExperiences {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by sensation.
    #[arg(long)]
    sensation: Option<SensationName>,
}

impl ListExperiences {
    pub(crate) async fn run(
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
            outcomes.emit(ListExperiencesOutcomes::Experiences(
                outcomes::ExperienceList(experiences),
            ));
        }

        Ok(outcomes)
    }
}
