use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum PeerCommands {
    Add(AddPeer),
    Get(GetPeer),
    List(ListPeers),
    Remove(RemovePeer),
}

impl PeerCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, PeerError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Add(addition) => addition.execute_request(&client).await?,
            Self::Get(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
            Self::Remove(removal) => removal.execute_request(&client).await?,
        };

        let response: PeerResponse = serde_json::from_slice(&bytes)?;
        Ok(PeerView::new(response).render().map(Into::into))
    }
}
