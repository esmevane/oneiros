use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Urge {
    pub name: UrgeName,
    pub description: Description,
    pub prompt: Prompt,
}

resource_name!(UrgeName);
