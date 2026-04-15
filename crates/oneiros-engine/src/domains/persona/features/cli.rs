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

        Ok(PersonaView::new(response).render().map(Into::into))
    }
}
