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

        let prompt = match &response {
            PeerResponse::Added(wrapped) => {
                PeerView::confirmed("added", &wrapped.data.name).to_string()
            }
            PeerResponse::Found(wrapped) => PeerView::detail(&wrapped.data).to_string(),
            PeerResponse::Listed(listed) => {
                let table = PeerView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            PeerResponse::Removed(id) => format!("Removed peer {id}"),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
