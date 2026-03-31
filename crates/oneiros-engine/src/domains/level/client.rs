use crate::*;

pub struct LevelClient<'a> {
    client: &'a Client,
}

impl<'a> LevelClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetLevel) -> Result<LevelResponse, ClientError> {
        self.client.put(&format!("/levels/{}", set.name), set).await
    }

    pub async fn get(&self, name: &LevelName) -> Result<LevelResponse, ClientError> {
        self.client.get(&format!("/levels/{name}")).await
    }

    pub async fn list(&self) -> Result<LevelResponse, ClientError> {
        self.client.get("/levels").await
    }

    pub async fn remove(&self, name: &LevelName) -> Result<LevelResponse, ClientError> {
        self.client.delete(&format!("/levels/{name}")).await
    }
}
