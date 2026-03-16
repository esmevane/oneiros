use clap::Args;
use oneiros_model::Persona;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListPersonasOutcomes {
    #[outcome(message("No personas configured."))]
    NoPersonas,

    #[outcome(message("Personas: {0:?}"))]
    Personas(Vec<Persona>),
}

#[derive(Clone, Args)]
pub struct ListPersonas;

impl ListPersonas {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ListPersonasOutcomes>, Vec<PressureSummary>), PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client.list_personas(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let personas: Vec<Persona> = response.data()?;

        if personas.is_empty() {
            outcomes.emit(ListPersonasOutcomes::NoPersonas);
        } else {
            outcomes.emit(ListPersonasOutcomes::Personas(personas));
        }

        Ok((outcomes, summaries))
    }
}
