mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemovePersonaOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemovePersona {
    /// The persona name to remove.
    name: PersonaName,
}

impl RemovePersona {
    pub(crate) async fn run(
        &self,
        context: Context,
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
