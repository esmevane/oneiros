use crate::*;

pub(crate) struct HostClient<'a> {
    client: &'a Client,
}

impl<'a> HostClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn init(&self, request: &InitHost) -> Result<HostResponse, ClientError> {
        self.client.post("/host", request).await
    }
}
