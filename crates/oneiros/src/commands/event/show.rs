use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowEventOutcomes {
    #[outcome(message("{0}"))]
    EventDetails(Event),
}

#[derive(Clone, Args)]
pub(crate) struct ShowEvent {
    /// The event id to display.
    id: EventId,
}

impl ShowEvent {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowEventOutcomes>, EventCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client.get_event(&context.ticket_token()?, &self.id).await?;
        outcomes.emit(ShowEventOutcomes::EventDetails(info));

        Ok(outcomes)
    }
}
