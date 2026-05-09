use clap::Subcommand;

use crate::*;

/// CLI subcommands for the brain domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub(crate) enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List(ListBrains),
}

impl BrainCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, BrainError> {
        let client = Client::from_config(config)?;
        let brain_client = BrainClient::new(&client);

        let response = match self {
            Self::Create(creation) => brain_client.create(creation).await?,
            Self::Get(lookup) => brain_client.get(lookup).await?,
            Self::List(listing) => brain_client.list(listing).await?,
        };

        Ok(BrainView::new(response).render().map(Into::into))
    }
}
