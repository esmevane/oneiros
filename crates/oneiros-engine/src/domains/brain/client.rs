//! HTTP client for the brain domain.

use crate::*;

/// Client scoped to brain operations.
pub struct BrainClient<'a> {
    client: &'a Client,
}

impl<'a> BrainClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new brain with the given name.
    pub async fn create(&self, creation: &CreateBrain) -> Result<BrainResponse, ClientError> {
        self.client.post("/brains", creation).await
    }

    /// Retrieve a single brain by key (name or ref).
    pub async fn get(&self, lookup: &GetBrain) -> Result<BrainResponse, ClientError> {
        let GetBrain::V1(lookup) = lookup;
        self.client.get(&format!("/brains/{}", lookup.key)).await
    }

    /// List all brains.
    pub async fn list(&self, listing: &ListBrains) -> Result<BrainResponse, ClientError> {
        let ListBrains::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/brains?{query}")).await
    }
}
