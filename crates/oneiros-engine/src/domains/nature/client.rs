use super::model::Nature;
use super::responses::NatureResponse;
use crate::client::{Client, ClientError};

pub struct NatureClient<'a> {
    client: &'a Client,
}

impl<'a> NatureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, nature: &Nature) -> Result<NatureResponse, ClientError> {
        self.client
            .put(&format!("/natures/{}", nature.name), nature)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<NatureResponse, ClientError> {
        self.client.get(&format!("/natures/{}", name)).await
    }

    pub async fn list(&self) -> Result<NatureResponse, ClientError> {
        self.client.get("/natures").await
    }

    pub async fn remove(&self, name: &str) -> Result<NatureResponse, ClientError> {
        self.client.delete(&format!("/natures/{}", name)).await
    }
}
