use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Nature {
    pub name: NatureName,
    pub description: String,
    pub prompt: String,
}

resource_name!(NatureName);
