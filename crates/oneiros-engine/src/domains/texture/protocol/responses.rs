use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TextureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureResponse {
    TextureSet(TextureSetResponse),
    TextureDetails(TextureDetailsResponse),
    Textures(TexturesResponse),
    NoTextures,
    TextureRemoved(TextureRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TextureSetResponse {
        V1 => { #[serde(flatten)] pub texture: Texture }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TextureDetailsResponse {
        V1 => { #[serde(flatten)] pub texture: Texture }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TexturesResponse {
        V1 => {
            pub items: Vec<Texture>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TextureRemovedResponse {
        V1 => {
            #[builder(into)] pub name: TextureName,
        }
    }
}
