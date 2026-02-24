use clap::Args;
use oneiros_client::Client;
use oneiros_model::SensationRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListSensationsOutcomes {
    #[outcome(message("No sensations configured."))]
    NoSensations,

    #[outcome(message("Sensations: {0:?}"))]
    Sensations(Vec<SensationRecord>),
}

#[derive(Clone, Args)]
pub struct ListSensations;

impl ListSensations {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListSensationsOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let sensations = client.list_sensations(&context.ticket_token()?).await?;

        if sensations.is_empty() {
            outcomes.emit(ListSensationsOutcomes::NoSensations);
        } else {
            outcomes.emit(ListSensationsOutcomes::Sensations(sensations));
        }

        Ok(outcomes)
    }
}
