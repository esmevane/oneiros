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

    pub(crate) async fn get(&self, id: &PeerId) -> Result<PeerResponse, ClientError> {
        self.client.get(&format!("/peers/{}", id)).await
    }

    pub(crate) async fn list(&self, request: &ListPeers) -> Result<PeerResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/peers?{query}")).await
    }

    pub(crate) async fn remove(&self, id: &PeerId) -> Result<PeerResponse, ClientError> {
        self.client.delete(&format!("/peers/{}", id)).await
    }
}
