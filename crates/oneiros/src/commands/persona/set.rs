use clap::Args;
use oneiros_model::PersonaName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetPersonaOutcomes {
    #[outcome(message("Persona '{0}' set."))]
    PersonaSet(PersonaName),
}

#[derive(Clone, Args)]
pub struct SetPersona {
    /// The persona name (identity).
    pub name: PersonaName,

    /// A human-readable description of the persona's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// The system prompt or instruction text for this persona.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetPersona {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetPersonaOutcomes>, PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info = client
            .set_persona(
                &context.ticket_token()?,
                Persona::init(
                    self.name.clone(),
                    self.description.clone(),
                    self.prompt.clone(),
                ),
            )
            .await?;
        outcomes.emit(SetPersonaOutcomes::PersonaSet(info.name.clone()));

        Ok(outcomes)
    }
}
