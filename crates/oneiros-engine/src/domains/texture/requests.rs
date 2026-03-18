use serde::{Deserialize, Serialize};

use super::model::Texture;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureRequest {
    Set(Texture),
    Get { name: String },
    List,
    Remove { name: String },
}
