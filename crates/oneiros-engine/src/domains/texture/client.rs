use crate::*;

pub struct TextureClient<'a> {
    client: &'a Client,
}

impl<'a> TextureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, set: &SetTexture) -> Result<TextureResponse, ClientError> {
        self.client
            .put(&format!("/textures/{}", set.name), set)
            .await
    }

    pub async fn get(&self, name: &TextureName) -> Result<TextureResponse, ClientError> {
        self.client.get(&format!("/textures/{name}")).await
    }

    pub async fn list(&self, request: &ListTextures) -> Result<TextureResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
        );
        self.client.get(&format!("/textures?{query}")).await
    }

    pub async fn remove(&self, name: &TextureName) -> Result<TextureResponse, ClientError> {
        self.client.delete(&format!("/textures/{name}")).await
    }
}
