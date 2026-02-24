use clap::Args;
use oneiros_client::Client;
use oneiros_model::SensationName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveSensationOutcomes {
    #[outcome(message("Sensation '{0}' removed."))]
    SensationRemoved(SensationName),
}

#[derive(Clone, Args)]
pub struct RemoveSensation {
    /// The sensation name to remove.
    name: SensationName,
}

impl RemoveSensation {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveSensationOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_sensation(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveSensationOutcomes::SensationRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
