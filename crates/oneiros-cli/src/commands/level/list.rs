use clap::Args;
use oneiros_model::Level;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListLevelsOutcomes {
    #[outcome(message("No levels configured."), prompt("No levels configured."))]
    NoLevels,

    #[outcome(message("Levels: {0:?}"), prompt("Levels: {0:?}"))]
    Levels(Vec<Level>),
}

#[derive(Clone, Args)]
pub struct ListLevels;

impl ListLevels {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ListLevelsOutcomes>, Vec<PressureSummary>), LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client.list_levels(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let levels: Vec<Level> = response.data()?;

        if levels.is_empty() {
            outcomes.emit(ListLevelsOutcomes::NoLevels);
        } else {
            outcomes.emit(ListLevelsOutcomes::Levels(levels));
        }

        Ok((outcomes, summaries))
    }
}
