use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TextureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum TextureResponse {
    TextureSet(TextureSetResponse),
    TextureDetails(TextureDetailsResponse),
    Textures(TexturesResponse),
    NoTextures,
    TextureRemoved(TextureRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TextureSetResponse {
        V1 => { #[serde(flatten)] pub(crate) texture: Texture }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TextureDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) texture: Texture }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TexturesResponse {
        V1 => {
            pub(crate) items: Vec<Texture>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TextureRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: TextureName,
        }
    }
}
