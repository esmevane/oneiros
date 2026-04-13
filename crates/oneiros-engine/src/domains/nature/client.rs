use crate::*;

pub(crate) struct NatureClient<'a> {
    client: &'a Client,
}

impl<'a> NatureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) async fn set(&self, set: &SetNature) -> Result<NatureResponse, ClientError> {
        self.client
            .put(&format!("/natures/{}", set.name), set)
            .await
    }

    pub(crate) async fn get(&self, name: &NatureName) -> Result<NatureResponse, ClientError> {
        self.client.get(&format!("/natures/{name}")).await
    }

    pub(crate) async fn list(&self, request: &ListNatures) -> Result<NatureResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/natures?{query}")).await
    }

    pub(crate) async fn remove(&self, name: &NatureName) -> Result<NatureResponse, ClientError> {
        self.client.delete(&format!("/natures/{name}")).await
    }
}
