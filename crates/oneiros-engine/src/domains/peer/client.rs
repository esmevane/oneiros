//! HTTP client for the peer domain.

use crate::*;

pub struct PeerClient<'a> {
    client: &'a Client,
}

impl<'a> PeerClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, add: &AddPeer) -> Result<PeerResponse, ClientError> {
        self.client.post("/peers", add).await
    }

    pub async fn get(&self, request: &GetPeer) -> Result<PeerResponse, ClientError> {
        self.client.get(&format!("/peers/{}", request.key)).await
    }

    pub async fn list(&self, request: &ListPeers) -> Result<PeerResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/peers?{query}")).await
    }

    pub async fn remove(&self, id: &PeerId) -> Result<PeerResponse, ClientError> {
        self.client.delete(&format!("/peers/{}", id)).await
    }
}
