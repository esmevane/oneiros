use crate::*;

pub(crate) struct SystemClient<'a> {
    client: &'a Client,
}

impl<'a> SystemClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn init(&self, request: &InitSystem) -> Result<SystemResponse, ClientError> {
        self.client.post("/system", request).await
    }
}
