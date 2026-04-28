use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum PeerCommands {
    Add(AddPeer),
    Get(GetPeer),
    List(ListPeers),
    Remove(RemovePeer),
}

impl PeerCommands {
    pub async fn execute(&self, context: &SystemContext) -> Result<Rendered<Responses>, PeerError> {
        let client = context.client();
        let peer_client = PeerClient::new(&client);

        let response = match self {
            Self::Add(addition) => peer_client.add(addition).await?,
            Self::Get(lookup) => peer_client.get(lookup).await?,
            Self::List(listing) => peer_client.list(listing).await?,
            Self::Remove(removal) => peer_client.remove(removal).await?,
        };

        Ok(PeerView::new(response).render().map(Into::into))
    }
}
