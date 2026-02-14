mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListPersonasOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListPersonas;

impl ListPersonas {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListPersonasOutcomes>, PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let personas = client.list_personas(&context.ticket_token()?).await?;

        if personas.is_empty() {
            outcomes.emit(ListPersonasOutcomes::NoPersonas);
        } else {
            outcomes.emit(ListPersonasOutcomes::Personas(personas));
        }

        Ok(outcomes)
    }
}
