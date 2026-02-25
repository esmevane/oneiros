use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ReplayProjectOutcomes {
    #[outcome(message("Replaying projections for brain."))]
    Replaying,
    #[outcome(message("Replayed {0} events."))]
    Replayed(usize),
}

#[derive(Clone, Args)]
pub struct ReplayProject;

impl ReplayProject {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ReplayProjectOutcomes>, ProjectCommandError> {
        let mut outcomes = Outcomes::new();

        outcomes.emit(ReplayProjectOutcomes::Replaying);

        let response = Client::new(context.socket_path())
            .replay_brain(&context.ticket_token()?)
            .await?;

        outcomes.emit(ReplayProjectOutcomes::Replayed(response.replayed));

        Ok(outcomes)
    }
}
