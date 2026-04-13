use crate::*;

pub(crate) struct ProjectClient<'a> {
    client: &'a Client,
}

impl<'a> ProjectClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn init(&self, request: &InitProject) -> Result<ProjectResponse, ClientError> {
        self.client.post("/projects", request).await
    }
}
