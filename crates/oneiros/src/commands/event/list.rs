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

    #[outcome(message("Events: {0:?}"))]
    Events(Vec<Event>),
}

#[derive(Clone, Args)]
pub(crate) struct ListEvents;

impl ListEvents {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListEventsOutcomes>, EventCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let textures = client.list_events(&context.ticket_token()?).await?;

        if textures.is_empty() {
            outcomes.emit(ListEventsOutcomes::NoEvents);
        } else {
            outcomes.emit(ListEventsOutcomes::Events(textures));
        }

        Ok(outcomes)
    }
}
