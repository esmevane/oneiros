//! HTTP client for the actor domain.

use crate::*;

/// Client scoped to actor operations.
pub(crate) struct ActorClient<'a> {
    client: &'a Client,
}

impl<'a> ActorClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new actor belonging to the given tenant.
    pub(crate) async fn create(&self, creation: &CreateActor) -> Result<ActorResponse, ClientError> {
        self.client.post("/actors", creation).await
    }

    /// Retrieve a single actor by ID.
    pub(crate) async fn get(&self, id: &ActorId) -> Result<ActorResponse, ClientError> {
        self.client.get(&format!("/actors/{}", id)).await
    }

    /// List all actors.
    pub(crate) async fn list(&self, request: &ListActors) -> Result<ActorResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/actors?{query}")).await
    }
}
