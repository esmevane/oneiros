use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ListEventsOutcomes {
    #[outcome(message("No events found."))]
    NoEvents,

    #[outcome(message("{0}"))]
    Event(Event),
}

#[derive(Clone, Args)]
pub struct ListEvents;

impl ListEvents {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListEventsOutcomes>, EventCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let events = client.list_events(&context.ticket_token()?).await?;

        if events.is_empty() {
            outcomes.emit(ListEventsOutcomes::NoEvents);
        } else {
            for event in events {
                outcomes.emit(ListEventsOutcomes::Event(event));
            }
        }

        Ok(outcomes)
    }
}
