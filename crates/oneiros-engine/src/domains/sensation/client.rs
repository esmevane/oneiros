use crate::*;

pub struct SensationClient<'a> {
    client: &'a Client,
}

impl<'a> SensationClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetSensation) -> Result<SensationResponse, ClientError> {
        self.client
            .put(&format!("/sensations/{}", set.name), set)
            .await
    }

    pub async fn get(&self, request: &GetSensation) -> Result<SensationResponse, ClientError> {
        self.client
            .get(&format!("/sensations/{}", request.key))
            .await
    }

    pub async fn list(&self, request: &ListSensations) -> Result<SensationResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/sensations?{query}")).await
    }

    pub async fn remove(&self, name: &SensationName) -> Result<SensationResponse, ClientError> {
        self.client.delete(&format!("/sensations/{name}")).await
    }
}
