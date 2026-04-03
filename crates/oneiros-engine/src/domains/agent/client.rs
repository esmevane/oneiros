use crate::*;

pub struct AgentClient<'a> {
    client: &'a Client,
}

impl<'a> AgentClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, creation: &CreateAgent) -> Result<AgentResponse, ClientError> {
        self.client.post("/agents", creation).await
    }

    pub async fn list(&self, request: &ListAgents) -> Result<AgentResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset
        );
        self.client.get(&format!("/agents?{query}")).await
    }

    pub async fn get(&self, name: &AgentName) -> Result<AgentResponse, ClientError> {
        self.client.get(&format!("/agents/{name}")).await
    }

    pub async fn update(&self, changes: &UpdateAgent) -> Result<AgentResponse, ClientError> {
        self.client
            .put(&format!("/agents/{name}", name = changes.name), changes)
            .await
    }

    pub async fn remove(&self, name: &AgentName) -> Result<AgentResponse, ClientError> {
        self.client.delete(&format!("/agents/{name}")).await
    }
}
