use crate::*;

pub(crate) struct SensationClient<'a> {
    client: &'a Client,
}

impl<'a> SensationClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, set: &SetSensation) -> Result<SensationResponse, ClientError> {
        self.client
            .put(&format!("/sensations/{}", set.name), set)
            .await
    }

    pub(crate) async fn get(&self, name: &SensationName) -> Result<SensationResponse, ClientError> {
        self.client.get(&format!("/sensations/{name}")).await
    }

    pub(crate) async fn list(&self, request: &ListSensations) -> Result<SensationResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/sensations?{query}")).await
    }

    pub(crate) async fn remove(&self, name: &SensationName) -> Result<SensationResponse, ClientError> {
        self.client.delete(&format!("/sensations/{name}")).await
    }
}
