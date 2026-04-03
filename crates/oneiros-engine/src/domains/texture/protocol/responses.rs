use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureResponse {
    TextureSet(TextureName),
    TextureDetails(Response<Texture>),
    Textures(Listed<Response<Texture>>),
    NoTextures,
    TextureRemoved(TextureName),
}
