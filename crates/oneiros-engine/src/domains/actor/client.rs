//! HTTP client for the actor domain.

use crate::*;

/// Client scoped to actor operations.
pub struct ActorClient<'a> {
    client: &'a Client,
}

impl<'a> ActorClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new actor belonging to the given tenant.
    pub async fn create(&self, creation: &CreateActor) -> Result<ActorResponse, ClientError> {
        self.client.post("/actors", creation).await
    }

    /// Retrieve a single actor by key (id or ref).
    pub async fn get(&self, request: &GetActor) -> Result<ActorResponse, ClientError> {
        self.client.get(&format!("/actors/{}", request.key)).await
    }

    /// List all actors.
    pub async fn list(&self, request: &ListActors) -> Result<ActorResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/actors?{query}")).await
    }
}
