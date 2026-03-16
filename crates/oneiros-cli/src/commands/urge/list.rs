use clap::Args;
use oneiros_model::Urge;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListUrgesOutcomes {
    #[outcome(message("No urges configured."))]
    NoUrges,

    #[outcome(message("Urges: {0:?}"))]
    Urges(Vec<Urge>),
}

#[derive(Clone, Args)]
pub struct ListUrges;

impl ListUrges {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ListUrgesOutcomes>, Vec<PressureSummary>), UrgeCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client.list_urges(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let urges: Vec<Urge> = response.data()?;

        if urges.is_empty() {
            outcomes.emit(ListUrgesOutcomes::NoUrges);
        } else {
            outcomes.emit(ListUrgesOutcomes::Urges(urges));
        }

        Ok((outcomes, summaries))
    }
}
