use clap::Args;
use oneiros_client::Client;
use oneiros_model::PersonaRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListPersonasOutcomes {
    #[outcome(message("No personas configured."))]
    NoPersonas,

    #[outcome(message("Personas: {0:?}"))]
    Personas(Vec<PersonaRecord>),
}

#[derive(Clone, Args)]
pub struct ListPersonas;

impl ListPersonas {
    pub async fn run(
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
