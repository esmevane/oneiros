//! HTTP client for lifecycle operations.

use crate::*;

/// Client scoped to lifecycle operations.
pub struct LifecycleClient<'a> {
    client: &'a Client,
}

impl<'a> LifecycleClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Run the dream lifecycle operation for the given agent.
    pub async fn dream(&self, agent: &AgentName) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/dream/{agent}"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the introspect lifecycle operation for the given agent.
    pub async fn introspect(
        &self,
        agent: &AgentName,
    ) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/introspect/{agent}"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the reflect lifecycle operation for the given agent.
    pub async fn reflect(&self, agent: &AgentName) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/reflect/{agent}"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the sense lifecycle operation for the given agent with the provided content.
    pub async fn sense(
        &self,
        agent: &AgentName,
        content: Content,
    ) -> Result<LifecycleResponse, ClientError> {
        #[derive(serde::Serialize)]
        struct Body {
            content: Content,
        }

        self.client
            .post(
                &format!("/sense/{agent}"),
                &Body { content },
            )
            .await
    }

    /// Run the sleep lifecycle operation for the given agent.
    pub async fn sleep(&self, agent: &AgentName) -> Result<LifecycleResponse, ClientError> {
        self.client
            .post(
                &format!("/sleep/{agent}"),
                &serde_json::Value::Null,
            )
            .await
    }
}
