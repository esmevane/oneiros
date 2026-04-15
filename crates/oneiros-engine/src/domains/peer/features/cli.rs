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
            PeerCommands::Add(add) => peer_client.add(add).await?,
            PeerCommands::Get(get) => peer_client.get(&get.id).await?,
            PeerCommands::List(list) => peer_client.list(list).await?,
            PeerCommands::Remove(remove) => peer_client.remove(&remove.id).await?,
        };

        Ok(PeerView::new(response).render().map(Into::into))
    }
}
