//! HTTP client for the tenant domain.

use crate::client::{Client, ClientError};

use super::responses::TenantResponse;

/// Client scoped to tenant operations.
pub struct TenantClient<'a> {
    client: &'a Client,
}

impl<'a> TenantClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new tenant with the given name.
    pub async fn create(&self, name: impl Into<String>) -> Result<TenantResponse, ClientError> {
        self.client
            .post("/tenants/", &serde_json::json!({ "name": name.into() }))
            .await
    }

    /// Retrieve a single tenant by ID.
    pub async fn get(&self, id: impl AsRef<str>) -> Result<TenantResponse, ClientError> {
        self.client.get(&format!("/tenants/{}", id.as_ref())).await
    }

    /// List all tenants.
    pub async fn list(&self) -> Result<TenantResponse, ClientError> {
        self.client.get("/tenants/").await
    }
}
