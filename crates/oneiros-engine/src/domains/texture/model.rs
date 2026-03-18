use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Texture {
    pub name: TextureName,
    pub description: String,
    pub prompt: String,
}

resource_name!(TextureName);
