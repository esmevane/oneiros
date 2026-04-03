use crate::*;

pub struct MemoryClient<'a> {
    client: &'a Client,
}

impl<'a> MemoryClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, request: &AddMemory) -> Result<MemoryResponse, ClientError> {
        self.client.post("/memories", request).await
    }

    pub async fn list(&self, request: &ListMemories) -> Result<MemoryResponse, ClientError> {
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(a) = &request.agent {
            params.push(("agent", a.to_string()));
        }

        params.push(("limit", request.filters.limit.to_string()));
        params.push(("offset", request.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");

        self.client.get(&format!("/memories?{query}")).await
    }

    pub async fn get(&self, request: &GetMemory) -> Result<MemoryResponse, ClientError> {
        self.client.get(&format!("/memories/{}", request.id)).await
    }
}
