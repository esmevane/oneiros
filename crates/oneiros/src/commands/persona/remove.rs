use clap::Args;
use oneiros_client::Client;
use oneiros_model::PersonaName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemovePersonaOutcomes {
    #[outcome(message("Persona '{0}' removed."))]
    PersonaRemoved(PersonaName),
}

#[derive(Clone, Args)]
pub struct RemovePersona {
    /// The persona name to remove.
    name: PersonaName,
}

impl RemovePersona {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemovePersonaOutcomes>, PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_persona(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemovePersonaOutcomes::PersonaRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
