use clap::Args;
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
pub struct ShowEvent {
    /// The event id to display.
    id: EventId,
}

impl ShowEvent {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowEventOutcomes>, EventCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info = client.get_event(&context.ticket_token()?, &self.id).await?;
        outcomes.emit(ShowEventOutcomes::EventDetails(info));

        Ok(outcomes)
    }
}
