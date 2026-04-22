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
    pub async fn create(&self, creation: &CreateTenant) -> Result<TenantResponse, ClientError> {
        self.client.post("/tenants", creation).await
    }

    /// Retrieve a single tenant by key (id or ref).
    pub async fn get(&self, request: &GetTenant) -> Result<TenantResponse, ClientError> {
        self.client.get(&format!("/tenants/{}", request.key)).await
    }

    /// List all tenants.
    pub async fn list(&self, request: &ListTenants) -> Result<TenantResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/tenants?{query}")).await
    }
}
