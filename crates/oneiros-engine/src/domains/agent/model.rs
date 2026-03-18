use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub persona: String,
    pub description: String,
    pub prompt: String,
}
