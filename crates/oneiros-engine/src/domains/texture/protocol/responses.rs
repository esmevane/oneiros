use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureResponse {
    TextureSet(TextureName),
    TextureDetails(Texture),
    Textures(Vec<Texture>),
    NoTextures,
    TextureRemoved(TextureName),
}
