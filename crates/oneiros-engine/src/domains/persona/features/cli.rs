use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
    Set(Persona),
    Show { name: PersonaName },
    List,
    Remove { name: PersonaName },
}

impl PersonaCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, PersonaError> {
        let client = context.client();
        let persona_client = PersonaClient::new(&client);

        let result = match self {
            PersonaCommands::Set(persona) => persona_client.set(persona).await?.into(),
            PersonaCommands::Show { name } => persona_client.get(name).await?.into(),
            PersonaCommands::List => persona_client.list().await?.into(),
            PersonaCommands::Remove { name } => persona_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
