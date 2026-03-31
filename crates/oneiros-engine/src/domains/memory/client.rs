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
        let path = match &request.agent {
            Some(a) => format!("/memories?agent={a}"),
            None => "/memories".to_string(),
        };

        self.client.get(&path).await
    }

    pub async fn get(&self, request: &GetMemory) -> Result<MemoryResponse, ClientError> {
        self.client.get(&format!("/memories/{}", request.id)).await
    }
}
