use clap::Args;
use oneiros_model::LevelName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveLevelOutcomes {
    #[outcome(message("Level '{0}' removed."), prompt("Level '{0}' removed."))]
    LevelRemoved(LevelName),
}

#[derive(Clone, Args, bon::Builder)]
pub struct RemoveLevel {
    /// The level name to remove.
    #[builder(into)]
    name: LevelName,
}

impl RemoveLevel {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<RemoveLevelOutcomes>, Vec<PressureSummary>), LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        client
            .remove_level(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveLevelOutcomes::LevelRemoved(self.name.clone()));

        Ok((outcomes, vec![]))
    }
}
