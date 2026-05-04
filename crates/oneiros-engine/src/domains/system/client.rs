use crate::*;

pub struct SystemClient<'a> {
    client: &'a Client,
}

impl<'a> SystemClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn init(&self, request: &InitSystem) -> Result<SystemResponse, ClientError> {
        self.client.post("/system", request).await
    }
}
