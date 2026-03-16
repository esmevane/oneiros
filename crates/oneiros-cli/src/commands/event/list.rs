use clap::Args;
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
    ) -> Result<(Outcomes<ListEventsOutcomes>, Vec<PressureSummary>), EventCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let response = client.list_events(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let events: Vec<Event> = response.data()?;

        if events.is_empty() {
            outcomes.emit(ListEventsOutcomes::NoEvents);
        } else {
            for event in events {
                outcomes.emit(ListEventsOutcomes::Event(event));
            }
        }

        Ok((outcomes, summaries))
    }
}
