use crate::*;

pub struct TextureClient<'a> {
    client: &'a Client,
}

impl<'a> TextureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn set(&self, setting: &SetTexture) -> Result<TextureResponse, ClientError> {
        let SetTexture::V1(body) = setting;
        self.client
            .put(&format!("/textures/{}", body.name), setting)
            .await
    }

    pub async fn get(&self, lookup: &GetTexture) -> Result<TextureResponse, ClientError> {
        let GetTexture::V1(lookup) = lookup;
        self.client.get(&format!("/textures/{}", lookup.key)).await
    }

    pub async fn list(&self, listing: &ListTextures) -> Result<TextureResponse, ClientError> {
        let ListTextures::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/textures?{query}")).await
    }

    pub async fn remove(&self, removal: &RemoveTexture) -> Result<TextureResponse, ClientError> {
        let RemoveTexture::V1(removal) = removal;
        self.client
            .delete(&format!("/textures/{}", removal.name))
            .await
    }
}
