use crate::client::{Client, ClientError};
use crate::*;

pub struct LevelClient<'a> {
    client: &'a Client,
}

impl<'a> LevelClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, level: &Level) -> Result<LevelResponse, ClientError> {
        self.client
            .put(&format!("/levels/{}", level.name), level)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<LevelResponse, ClientError> {
        self.client.get(&format!("/levels/{}", name)).await
    }

    pub async fn list(&self) -> Result<LevelResponse, ClientError> {
        self.client.get("/levels").await
    }

    pub async fn remove(&self, name: &str) -> Result<LevelResponse, ClientError> {
        self.client.delete(&format!("/levels/{}", name)).await
    }
}
