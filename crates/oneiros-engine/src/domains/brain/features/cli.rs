use clap::Subcommand;

use crate::*;

/// CLI subcommands for the brain domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List(ListBrains),
}

impl BrainCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, BrainError> {
        let client = context.client();
        let brain_client = BrainClient::new(&client);

        let response = match self {
            Self::Create(creation) => brain_client.create(creation).await?,
            Self::Get(lookup) => brain_client.get(lookup).await?,
            Self::List(listing) => brain_client.list(listing).await?,
        };

        Ok(BrainView::new(response).render().map(Into::into))
    }
}
