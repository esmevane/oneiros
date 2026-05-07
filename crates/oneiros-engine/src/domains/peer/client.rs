//! HTTP client for the peer domain.

use crate::*;

pub(crate) struct PeerClient<'a> {
    client: &'a Client,
}

impl<'a> PeerClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn add(&self, add: &AddPeer) -> Result<PeerResponse, ClientError> {
        self.client.post("/peers", add).await
    }

    pub(crate) async fn get(&self, lookup: &GetPeer) -> Result<PeerResponse, ClientError> {
        let GetPeer::V1(lookup) = lookup;
        self.client.get(&format!("/peers/{}", lookup.key)).await
    }

    pub(crate) async fn list(&self, listing: &ListPeers) -> Result<PeerResponse, ClientError> {
        let ListPeers::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/peers?{query}")).await
    }

    pub(crate) async fn remove(&self, removal: &RemovePeer) -> Result<PeerResponse, ClientError> {
        let RemovePeer::V1(removal) = removal;
        self.client.delete(&format!("/peers/{}", removal.id)).await
    }
}
