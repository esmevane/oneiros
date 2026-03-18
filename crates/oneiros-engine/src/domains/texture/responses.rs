use serde::{Deserialize, Serialize};

use super::model::Texture;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureResponse {
    Set(Texture),
    Found(Texture),
    Listed(Vec<Texture>),
    Removed,
}
