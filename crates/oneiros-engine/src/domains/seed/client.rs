use crate::*;

pub struct SeedClient<'a> {
    client: &'a Client,
}

impl<'a> SeedClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn core(&self) -> Result<SeedResponse, ClientError> {
        self.client.post("/seed/core", &()).await
    }

    pub async fn agents(&self) -> Result<SeedResponse, ClientError> {
        self.client.post("/seed/agents", &()).await
    }
}
