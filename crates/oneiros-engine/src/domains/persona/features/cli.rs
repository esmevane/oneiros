use clap::Subcommand;

use crate::*;

/// CLI subcommands for the persona domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
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
            Self::Set(setting) => persona_client.set(setting).await?,
            Self::Show(lookup) => persona_client.get(lookup).await?,
            Self::List(listing) => persona_client.list(listing).await?,
            Self::Remove(removal) => persona_client.remove(removal).await?,
        };

        Ok(PersonaView::new(response).render().map(Into::into))
    }
}
