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
            PersonaResponse::PersonaSet(name) => PersonaView::confirmed("set", name).to_string(),
            PersonaResponse::PersonaDetails(wrapped) => {
                PersonaView::detail(&wrapped.data).to_string()
            }
            PersonaResponse::Personas(listed) => {
                let table = PersonaView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            PersonaResponse::NoPersonas => format!("{}", "No personas configured.".muted()),
            PersonaResponse::PersonaRemoved(name) => {
                PersonaView::confirmed("removed", name).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
