use crate::*;

pub struct ProjectClient<'a> {
    client: &'a Client,
}

impl<'a> ProjectClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn init(&self, request: &InitProject) -> Result<ProjectResponse, ClientError> {
        self.client.post("/projects", request).await
    }
}
