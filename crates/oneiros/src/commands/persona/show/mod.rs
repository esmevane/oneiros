mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowPersonaOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowPersona {
    /// The persona name to display.
    name: PersonaName,
}

impl ShowPersona {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowPersonaOutcomes>, PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_persona(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowPersonaOutcomes::PersonaDetails(info));

        Ok(outcomes)
    }
}
