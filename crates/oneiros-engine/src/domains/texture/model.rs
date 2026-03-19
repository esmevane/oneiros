use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Texture {
    pub name: TextureName,
    pub description: Description,
    pub prompt: Prompt,
}

resource_name!(TextureName);
