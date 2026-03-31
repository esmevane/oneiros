use crate::*;

pub struct NatureClient<'a> {
    client: &'a Client,
}

impl<'a> NatureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetNature) -> Result<NatureResponse, ClientError> {
        self.client
            .put(&format!("/natures/{}", set.name), set)
            .await
    }

    pub async fn get(&self, name: &NatureName) -> Result<NatureResponse, ClientError> {
        self.client.get(&format!("/natures/{name}")).await
    }

    pub async fn list(&self) -> Result<NatureResponse, ClientError> {
        self.client.get("/natures").await
    }

    pub async fn remove(&self, name: &NatureName) -> Result<NatureResponse, ClientError> {
        self.client.delete(&format!("/natures/{name}")).await
    }
}
