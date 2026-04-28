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
    pub async fn get(&self, lookup: &GetActor) -> Result<ActorResponse, ClientError> {
        let GetActor::V1(lookup) = lookup;
        self.client.get(&format!("/actors/{}", lookup.key)).await
    }

    /// List all actors.
    pub async fn list(&self, listing: &ListActors) -> Result<ActorResponse, ClientError> {
        let ListActors::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/actors?{query}")).await
    }
}
