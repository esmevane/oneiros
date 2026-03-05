use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectTextureByName {
    pub name: TextureName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(Texture),
    TextureRemoved(SelectTextureByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureRequests {
    SetTexture(Texture),
    RemoveTexture(SelectTextureByName),
    GetTexture(SelectTextureByName),
    ListTextures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureResponses {
    TextureSet(Texture),
    TextureFound(Texture),
    TexturesListed(Vec<Texture>),
    TextureRemoved,
}
