use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Level {
    pub name: LevelName,
    pub description: String,
    pub prompt: String,
}

resource_name!(LevelName);
