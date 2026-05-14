use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SetTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TextureName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub(crate) prompt: Prompt,
        }
    }
}

impl ClientRequest for SetTexture {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let SetTexture::V1(body) = self;
        client.put(&format!("/textures/{}", body.name), self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TextureName>,
        }
    }
}

impl ClientRequest for GetTexture {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetTexture::V1(lookup) = self;
        client.get(&format!("/textures/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RemoveTexture {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: TextureName,
        }
    }
}

impl ClientRequest for RemoveTexture {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let RemoveTexture::V1(removal) = self;
        client.delete(&format!("/textures/{}", removal.name)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListTextures {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListTextures {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListTextures::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/textures?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TextureRequestType, display = "kebab-case")]
pub(crate) enum TextureRequest {
    SetTexture(SetTexture),
    GetTexture(GetTexture),
    ListTextures(ListTextures),
    RemoveTexture(RemoveTexture),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TextureRequestType::SetTexture, "set-texture"),
            (TextureRequestType::GetTexture, "get-texture"),
            (TextureRequestType::ListTextures, "list-textures"),
            (TextureRequestType::RemoveTexture, "remove-texture"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
