use clap::Args;
use oneiros_model::UrgeName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveUrgeOutcomes {
    #[outcome(message("Urge '{0}' removed."))]
    UrgeRemoved(UrgeName),
}

#[derive(Clone, Args)]
pub struct RemoveUrge {
    /// The urge name to remove.
    name: UrgeName,
}

impl RemoveUrge {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<RemoveUrgeOutcomes>, Vec<PressureSummary>), UrgeCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        client
            .remove_urge(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveUrgeOutcomes::UrgeRemoved(self.name.clone()));

        Ok((outcomes, vec![]))
    }
}
