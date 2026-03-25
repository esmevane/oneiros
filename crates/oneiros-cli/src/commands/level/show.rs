use clap::Args;
use oneiros_model::Level;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowLevelOutcomes {
    #[outcome(message("Level '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt), prompt("Level '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    LevelDetails(Level),
}

#[derive(Clone, Args, bon::Builder)]
pub struct ShowLevel {
    /// The level name to display.
    #[builder(into)]
    name: LevelName,
}

impl ShowLevel {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ShowLevelOutcomes>, Vec<PressureSummary>), LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_level(&context.ticket_token()?, &self.name)
            .await?;
        let summaries = response.pressure_summaries();
        let info: Level = response.data()?;
        outcomes.emit(ShowLevelOutcomes::LevelDetails(info));

        Ok((outcomes, summaries))
    }
}
