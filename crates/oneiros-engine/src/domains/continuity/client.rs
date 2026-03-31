//! HTTP client for continuity operations.

use crate::*;

/// Client scoped to continuity operations.
pub struct ContinuityClient<'a> {
    client: &'a Client,
}

impl<'a> ContinuityClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Emerge — create and activate an agent's continuity.
    pub async fn emerge(&self, body: &EmergeAgent) -> Result<ContinuityResponse, ClientError> {
        self.client.post("/continuity", body).await
    }

    /// Recede — end an agent's continuity.
    pub async fn recede(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client.delete(&format!("/continuity/{agent}")).await
    }

    /// Status — read the current state of an agent's continuity.
    pub async fn status(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client.get(&format!("/continuity/{agent}")).await
    }

    /// Wake an agent.
    pub async fn wake(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/wake"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Retrieve the guidebook for an agent.
    pub async fn guidebook(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .get(&format!("/continuity/{agent}/guidebook"))
            .await
    }

    /// Run the dream continuity operation for the given agent.
    pub async fn dream(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/dream"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the introspect continuity operation for the given agent.
    pub async fn introspect(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/introspect"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the reflect continuity operation for the given agent.
    pub async fn reflect(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/reflect"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the sense continuity operation for the given agent with the provided content.
    pub async fn sense(&self, selector: &SenseContent) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/sense", agent = selector.agent),
                selector,
            )
            .await
    }

    /// Run the sleep continuity operation for the given agent.
    pub async fn sleep(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/sleep"),
                &serde_json::Value::Null,
            )
            .await
    }
}
