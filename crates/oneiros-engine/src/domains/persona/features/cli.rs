use clap::Subcommand;

use crate::*;

/// CLI subcommands for the persona domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub(crate) enum PersonaCommands {
    Set(SetPersona),
    Show(GetPersona),
    List(ListPersonas),
    Remove(RemovePersona),
}

impl PersonaCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, PersonaError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Set(setting) => setting.execute_request(&client).await?,
            Self::Show(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
            Self::Remove(removal) => removal.execute_request(&client).await?,
        };

        let response: PersonaResponse = serde_json::from_slice(&bytes)?;
        Ok(PersonaView::new(response).render().map(Into::into))
    }
}
