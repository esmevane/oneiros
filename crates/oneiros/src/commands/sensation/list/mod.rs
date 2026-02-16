mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListSensationsOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListSensations;

impl ListSensations {
    pub(crate) async fn run(
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
