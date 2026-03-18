//! HTTP client for lifecycle operations.

use crate::client::{Client, ClientError};

use super::responses::LifecycleResponse;

/// Client scoped to lifecycle operations.
pub struct LifecycleClient<'a> {
    client: &'a Client,
}

impl<'a> LifecycleClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Run the dream lifecycle operation for the given agent.
    pub async fn dream(&self, agent: impl AsRef<str>) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/dream/{}", agent.as_ref()),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the introspect lifecycle operation for the given agent.
    pub async fn introspect(
        &self,
        agent: impl AsRef<str>,
    ) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/introspect/{}", agent.as_ref()),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the reflect lifecycle operation for the given agent.
    pub async fn reflect(&self, agent: impl AsRef<str>) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/reflect/{}", agent.as_ref()),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the sense lifecycle operation for the given agent with the provided content.
    pub async fn sense(
        &self,
        agent: impl AsRef<str>,
        content: impl Into<String>,
    ) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/sense/{}", agent.as_ref()),
                &serde_json::json!({ "content": content.into() }),
            )
            .await
    }

    /// Run the sleep lifecycle operation for the given agent.
    pub async fn sleep(&self, agent: impl AsRef<str>) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/sleep/{}", agent.as_ref()),
                &serde_json::Value::Null,
            )
            .await
    }
}
