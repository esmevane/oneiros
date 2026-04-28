use clap::Subcommand;

use crate::*;

/// CLI subcommands for the nature domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub enum NatureCommands {
    Set(SetNature),
    Show(GetNature),
    List(ListNatures),
    Remove(RemoveNature),
}

impl NatureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, NatureError> {
        let client = context.client();
        let nature_client = NatureClient::new(&client);

        let response = match self {
            Self::Set(setting) => nature_client.set(setting).await?,
            Self::Show(lookup) => nature_client.get(lookup).await?,
            Self::List(listing) => nature_client.list(listing).await?,
            Self::Remove(removal) => nature_client.remove(removal).await?,
        };

        Ok(NatureView::new(response).render().map(Into::into))
    }
}
