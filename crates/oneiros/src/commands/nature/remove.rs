use clap::Args;
use oneiros_client::Client;
use oneiros_model::NatureName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveNatureOutcomes {
    #[outcome(message("Nature '{0}' removed."))]
    NatureRemoved(NatureName),
}

#[derive(Clone, Args)]
pub struct RemoveNature {
    /// The nature name to remove.
    name: NatureName,
}

impl RemoveNature {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveNatureOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_nature(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveNatureOutcomes::NatureRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
