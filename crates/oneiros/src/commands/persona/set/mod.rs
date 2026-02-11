mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetPersonaOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetPersona {
    /// The persona name (identity).
    name: PersonaName,

    /// A human-readable description of the persona's purpose.
    #[arg(long, default_value = "")]
    description: Description,

    /// The system prompt or instruction text for this persona.
    #[arg(long, default_value = "")]
    prompt: Prompt,
}

impl SetPersona {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<SetPersonaOutcomes>, PersonaCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_persona(
                &context.ticket_token()?,
                Persona {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(SetPersonaOutcomes::PersonaSet(info.name));

        Ok(outcomes)
    }
}
