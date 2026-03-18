use super::model::Sensation;
use super::responses::SensationResponse;
use crate::client::{Client, ClientError};

pub struct SensationClient<'a> {
    client: &'a Client,
}

impl<'a> SensationClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, sensation: &Sensation) -> Result<SensationResponse, ClientError> {
        self.client
            .put(&format!("/sensations/{}", sensation.name), sensation)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<SensationResponse, ClientError> {
        self.client.get(&format!("/sensations/{}", name)).await
    }

    pub async fn list(&self) -> Result<SensationResponse, ClientError> {
        self.client.get("/sensations").await
    }

    pub async fn remove(&self, name: &str) -> Result<SensationResponse, ClientError> {
        self.client.delete(&format!("/sensations/{}", name)).await
    }
}
