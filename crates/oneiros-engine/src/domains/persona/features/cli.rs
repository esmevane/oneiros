use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
    Set(SetPersona),
    Show(GetPersona),
    List(ListPersonas),
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
            PersonaCommands::Set(set) => persona_client.set(set).await?,
            PersonaCommands::Show(get) => persona_client.get(&get.name).await?,
            PersonaCommands::List(list) => persona_client.list(list).await?,
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
            PersonaResponse::Personas(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for persona in &listed.items {
                    out.push_str(&format!("  {} — {}\n\n", persona.name, persona.description,));
                }
                out
            }
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
