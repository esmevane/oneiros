use clap::Subcommand;

use crate::*;

/// CLI subcommands for the urge domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
    Set(SetUrge),
    Show(GetUrge),
    List(ListUrges),
    Remove(RemoveUrge),
}

impl UrgeCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, UrgeError> {
        let client = context.client();
        let urge_client = UrgeClient::new(&client);

        let response = match self {
            Self::Set(setting) => urge_client.set(setting).await?,
            Self::Show(lookup) => urge_client.get(lookup).await?,
            Self::List(listing) => urge_client.list(listing).await?,
            Self::Remove(removal) => urge_client.remove(removal).await?,
        };

        Ok(UrgeView::new(response).render().map(Into::into))
    }
}
