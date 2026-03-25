use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(Texture),
    TextureRemoved(TextureRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureRemoved {
    pub name: TextureName,
}
