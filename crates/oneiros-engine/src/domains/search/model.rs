use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SearchResult {
    pub kind: String,
    pub id: String,
    pub content: String,
    pub rank: f64,
}
