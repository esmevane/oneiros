use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = TextureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureResponse {
    TextureSet(TextureName),
    TextureDetails(Response<Texture>),
    Textures(Listed<Response<Texture>>),
    NoTextures,
    TextureRemoved(TextureName),
}
