//! HTTP client for the tenant domain.

use crate::*;

/// Client scoped to tenant operations.
pub(crate) struct TenantClient<'a> {
    client: &'a Client,
}

impl<'a> TenantClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new tenant with the given name.
    pub(crate) async fn create(&self, creation: &CreateTenant) -> Result<TenantResponse, ClientError> {
        self.client.post("/tenants", creation).await
    }

    /// Retrieve a single tenant by ID.
    pub(crate) async fn get(&self, id: &TenantId) -> Result<TenantResponse, ClientError> {
        self.client.get(&format!("/tenants/{}", id)).await
    }

    /// List all tenants.
    pub(crate) async fn list(&self, request: &ListTenants) -> Result<TenantResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/tenants?{query}")).await
    }
}
