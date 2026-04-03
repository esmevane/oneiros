use crate::*;

pub struct UrgeClient<'a> {
    client: &'a Client,
}

impl<'a> UrgeClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetUrge) -> Result<UrgeResponse, ClientError> {
        self.client.put(&format!("/urges/{}", set.name), set).await
    }

    pub async fn get(&self, name: &UrgeName) -> Result<UrgeResponse, ClientError> {
        self.client.get(&format!("/urges/{name}")).await
    }

    pub async fn list(&self, request: &ListUrges) -> Result<UrgeResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/urges?{query}")).await
    }

    pub async fn remove(&self, name: &UrgeName) -> Result<UrgeResponse, ClientError> {
        self.client.delete(&format!("/urges/{name}")).await
    }
}
