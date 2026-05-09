//! HTTP client for continuity operations.

use crate::*;

/// Client scoped to continuity operations.
pub(crate) struct ContinuityClient<'a> {
    client: &'a Client,
}

impl<'a> ContinuityClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Emerge — create and activate an agent's continuity.
    pub(crate) async fn emerge(
        &self,
        body: &EmergeAgent,
    ) -> Result<ContinuityResponse, ClientError> {
        self.client.post("/continuity", body).await
    }

    /// Recede — end an agent's continuity.
    pub(crate) async fn recede(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        self.client.delete(&format!("/continuity/{agent}")).await
    }

    /// Status — cross-agent activity overview.
    pub(crate) async fn status(&self) -> Result<ContinuityResponse, ClientError> {
        self.client.get("/continuity").await
    }

    /// Wake an agent.
    pub(crate) async fn wake(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/wake"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Retrieve the guidebook for an agent.
    pub(crate) async fn guidebook(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        self.client
            .get(&format!("/continuity/{agent}/guidebook"))
            .await
    }

    /// Run the dream continuity operation for the given agent.
    pub(crate) async fn dream(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.dream_with(agent, &DreamOverrides::default()).await
    }

    /// Run the dream continuity operation with explicit per-request overrides.
    ///
    /// Overrides serialize into the URL query string. Only `Some(_)` fields
    /// are emitted, so passing `DreamOverrides::default()` is equivalent to
    /// `dream(agent)`.
    pub(crate) async fn dream_with(
        &self,
        agent: &AgentName,
        overrides: &DreamOverrides,
    ) -> Result<ContinuityResponse, ClientError> {
        let query = encode_dream_overrides(overrides);
        let path = if query.is_empty() {
            format!("/continuity/{agent}/dream")
        } else {
            format!("/continuity/{agent}/dream?{query}")
        };
        self.client.post(&path, &serde_json::Value::Null).await
    }

    /// Run the introspect continuity operation for the given agent.
    pub(crate) async fn introspect(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/introspect"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the reflect continuity operation for the given agent.
    pub(crate) async fn reflect(
        &self,
        agent: &AgentName,
    ) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/reflect"),
                &serde_json::Value::Null,
            )
            .await
    }

    /// Run the sense continuity operation for the given agent with the provided content.
    pub(crate) async fn sense(
        &self,
        sensing: &SenseContent,
    ) -> Result<ContinuityResponse, ClientError> {
        let SenseContent::V1(sense) = sensing;
        self.client
            .post(
                &format!("/continuity/{agent}/sense", agent = sense.agent),
                sensing,
            )
            .await
    }

    /// Run the sleep continuity operation for the given agent.
    pub(crate) async fn sleep(&self, agent: &AgentName) -> Result<ContinuityResponse, ClientError> {
        self.client
            .post(
                &format!("/continuity/{agent}/sleep"),
                &serde_json::Value::Null,
            )
            .await
    }
}

fn encode_dream_overrides(overrides: &DreamOverrides) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(value) = overrides.recent_window {
        parts.push(format!("recent_window={value}"));
    }
    if let Some(value) = overrides.dream_depth {
        parts.push(format!("dream_depth={value}"));
    }
    if let Some(value) = overrides.cognition_size {
        parts.push(format!("cognition_size={value}"));
    }
    if let Some(value) = &overrides.recollection_level {
        parts.push(format!("recollection_level={value}"));
    }
    if let Some(value) = overrides.recollection_size {
        parts.push(format!("recollection_size={value}"));
    }
    if let Some(value) = overrides.experience_size {
        parts.push(format!("experience_size={value}"));
    }
    parts.join("&")
}
