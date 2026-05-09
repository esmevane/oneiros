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
    pub(crate) async fn create(
        &self,
        creation: &CreateTenant,
    ) -> Result<TenantResponse, ClientError> {
        self.client.post("/tenants", creation).await
    }

    /// Retrieve a single tenant by key (id or ref).
    pub(crate) async fn get(&self, lookup: &GetTenant) -> Result<TenantResponse, ClientError> {
        let GetTenant::V1(lookup) = lookup;
        self.client.get(&format!("/tenants/{}", lookup.key)).await
    }

    /// List all tenants.
    pub(crate) async fn list(&self, listing: &ListTenants) -> Result<TenantResponse, ClientError> {
        let ListTenants::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/tenants?{query}")).await
    }
}
