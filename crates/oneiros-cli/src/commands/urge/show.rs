use clap::Args;
use oneiros_model::Urge;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowUrgeOutcomes {
    #[outcome(message("Urge '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    UrgeDetails(Urge),
}

#[derive(Clone, Args)]
pub struct ShowUrge {
    /// The urge name to display.
    name: UrgeName,
}

impl ShowUrge {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ShowUrgeOutcomes>, Vec<PressureSummary>), UrgeCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_urge(&context.ticket_token()?, &self.name)
            .await?;
        let summaries = response.pressure_summaries();
        let info: Urge = response.data()?;
        outcomes.emit(ShowUrgeOutcomes::UrgeDetails(info));

        Ok((outcomes, summaries))
    }
}
