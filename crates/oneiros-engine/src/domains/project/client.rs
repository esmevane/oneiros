use crate::*;

pub struct ProjectClient<'a> {
    client: &'a Client,
}

impl<'a> ProjectClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn init(&self, brain_name: &BrainName) -> Result<ProjectResponse, ClientError> {
        self.client
            .post(
                "/project/init",
                &serde_json::json!({ "brain_name": brain_name }),
            )
            .await
    }
}
