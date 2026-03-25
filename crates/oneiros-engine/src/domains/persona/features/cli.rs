use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
    Set(Persona),
    Show(GetPersona),
    List,
    Remove(RemovePersona),
}

impl PersonaCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, PersonaError> {
        let client = context.client();
        let persona_client = PersonaClient::new(&client);

        let response = match self {
            PersonaCommands::Set(persona) => persona_client.set(persona).await?,
            PersonaCommands::Show(get) => persona_client.get(&get.name).await?,
            PersonaCommands::List => persona_client.list().await?,
            PersonaCommands::Remove(removal) => persona_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            PersonaResponse::PersonaSet(name) => format!("Persona '{name}' set."),
            PersonaResponse::PersonaDetails(p) => {
                format!(
                    "Persona '{}'\n  Description: {}\n  Prompt: {}",
                    p.name, p.description, p.prompt
                )
            }
            PersonaResponse::Personas(personas) => format!("Personas: {personas:?}"),
            PersonaResponse::NoPersonas => "No personas configured.".to_string(),
            PersonaResponse::PersonaRemoved(name) => format!("Persona '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
