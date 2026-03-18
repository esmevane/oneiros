use super::model::Texture;
use super::responses::TextureResponse;
use crate::client::{Client, ClientError};

pub struct TextureClient<'a> {
    client: &'a Client,
}

impl<'a> TextureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, texture: &Texture) -> Result<TextureResponse, ClientError> {
        self.client
            .put(&format!("/textures/{}", texture.name), texture)
            .await
    }

    pub async fn get(&self, name: &str) -> Result<TextureResponse, ClientError> {
        self.client.get(&format!("/textures/{}", name)).await
    }

    pub async fn list(&self) -> Result<TextureResponse, ClientError> {
        self.client.get("/textures").await
    }

    pub async fn remove(&self, name: &str) -> Result<TextureResponse, ClientError> {
        self.client.delete(&format!("/textures/{}", name)).await
    }
}
