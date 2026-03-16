use clap::Args;
use oneiros_model::Persona;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowPersonaOutcomes {
    #[outcome(message("Persona '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    PersonaDetails(Persona),
}

#[derive(Clone, Args)]
pub struct ShowPersona {
    /// The persona name to display.
    name: PersonaName,
}

impl ShowPersona {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ShowPersonaOutcomes>, Vec<PressureSummary>), PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_persona(&context.ticket_token()?, &self.name)
            .await?;
        let summaries = response.pressure_summaries();
        let info: Persona = response.data()?;
        outcomes.emit(ShowPersonaOutcomes::PersonaDetails(info));

        Ok((outcomes, summaries))
    }
}
