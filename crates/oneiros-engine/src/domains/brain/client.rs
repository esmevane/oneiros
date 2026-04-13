//! HTTP client for the brain domain.

use crate::*;

/// Client scoped to brain operations.
pub(crate) struct BrainClient<'a> {
    client: &'a Client,
}

impl<'a> BrainClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Create a new brain with the given name.
    pub(crate) async fn create(&self, creation: &CreateBrain) -> Result<BrainResponse, ClientError> {
        self.client.post("/brains", creation).await
    }

    /// Retrieve a single brain by name.
    pub(crate) async fn get(&self, name: &BrainName) -> Result<BrainResponse, ClientError> {
        self.client.get(&format!("/brains/{}", name)).await
    }

    /// List all brains.
    pub(crate) async fn list(&self, request: &ListBrains) -> Result<BrainResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/brains?{query}")).await
    }
}
