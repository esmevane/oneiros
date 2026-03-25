use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetTexture {
    pub name: TextureName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveTexture {
    pub name: TextureName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TextureRequest {
    Set(Texture),
    Get(GetTexture),
    List,
    Remove(RemoveTexture),
}
