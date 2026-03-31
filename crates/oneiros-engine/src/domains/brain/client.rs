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

    /// Retrieve a single brain by name.
    pub async fn get(&self, name: &BrainName) -> Result<BrainResponse, ClientError> {
        self.client.get(&format!("/brains/{}", name)).await
    }

    /// List all brains.
    pub async fn list(&self) -> Result<BrainResponse, ClientError> {
        self.client.get("/brains").await
    }
}
