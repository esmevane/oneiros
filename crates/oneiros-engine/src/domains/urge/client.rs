use crate::*;
use crate::client::{Client, ClientError};

pub struct UrgeClient<'a> {
    client: &'a Client,
}

impl<'a> UrgeClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, urge: &Urge) -> Result<UrgeResponse, ClientError> {
        self.client
            .put(&format!("/urges/{}", urge.name), urge)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<UrgeResponse, ClientError> {
        self.client.get(&format!("/urges/{}", name)).await
    }

    pub async fn list(&self) -> Result<UrgeResponse, ClientError> {
        self.client.get("/urges").await
    }

    pub async fn remove(&self, name: &str) -> Result<UrgeResponse, ClientError> {
        self.client.delete(&format!("/urges/{}", name)).await
    }
}
