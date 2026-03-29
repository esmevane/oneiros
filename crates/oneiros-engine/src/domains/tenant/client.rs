//! HTTP client for the tenant domain.

use crate::*;

/// Client scoped to tenant operations.
pub struct TenantClient<'a> {
    client: &'a Client,
}

impl<'a> TenantClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new tenant with the given name.
    pub async fn create(&self, name: &TenantName) -> Result<TenantResponse, ClientError> {
        self.client
            .post("/tenants/", &serde_json::json!({ "name": name }))
            .await
    }

    /// Retrieve a single tenant by ID.
    pub async fn get(&self, id: &TenantId) -> Result<TenantResponse, ClientError> {
        self.client.get(&format!("/tenants/{}", id)).await
    }

    /// List all tenants.
    pub async fn list(&self) -> Result<TenantResponse, ClientError> {
        self.client.get("/tenants/").await
    }
}
