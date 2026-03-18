//! HTTP client for the actor domain.

use crate::client::{Client, ClientError};

use super::responses::ActorResponse;

/// Client scoped to actor operations.
pub struct ActorClient<'a> {
    client: &'a Client,
}

impl<'a> ActorClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new actor belonging to the given tenant.
    pub async fn create(
        &self,
        tenant_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<ActorResponse, ClientError> {
        self.client
            .post(
                "/actors/",
                &serde_json::json!({ "tenant_id": tenant_id.into(), "name": name.into() }),
            )
            .await
    }

    /// Retrieve a single actor by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<ActorResponse, ClientError> {
        self.client.get(&format!("/actors/{}", id.as_ref())).await
    }

    /// List all actors.
    pub async fn list(&self) -> Result<ActorResponse, ClientError> {
        self.client.get("/actors/").await
    }
}
