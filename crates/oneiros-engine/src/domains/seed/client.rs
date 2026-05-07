use crate::*;

pub(crate) struct SeedClient<'a> {
    client: &'a Client,
}

impl<'a> SeedClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn core(&self) -> Result<SeedResponse, ClientError> {
        self.client.post("/seed/core", &()).await
    }

    pub(crate) async fn agents(&self) -> Result<SeedResponse, ClientError> {
        self.client.post("/seed/agents", &()).await
    }
}
