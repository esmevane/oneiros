use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AddPeer {
        #[derive(clap::Args)]
        V1 => {
            #[arg(id = "peer_address")]
            pub(crate) address: String,
            #[arg(long)]
            pub(crate) name: Option<String>,
        }
    }
}

impl ClientRequest for AddPeer {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/peers", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetPeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<PeerId>,
        }
    }
}

impl ClientRequest for GetPeer {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetPeer::V1(lookup) = self;
        client.get(&format!("/peers/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemovePeer {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: PeerId,
        }
    }
}

impl ClientRequest for RemovePeer {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemovePeer::V1(removal) = self;
        client.delete(&format!("/peers/{}", removal.id)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListPeers {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListPeers {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListPeers::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/peers?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PeerRequestType, display = "kebab-case")]
pub(crate) enum PeerRequest {
    AddPeer(AddPeer),
    GetPeer(GetPeer),
    RemovePeer(RemovePeer),
    ListPeers(ListPeers),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PeerRequestType::AddPeer, "add-peer"),
            (PeerRequestType::GetPeer, "get-peer"),
            (PeerRequestType::RemovePeer, "remove-peer"),
            (PeerRequestType::ListPeers, "list-peers"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
