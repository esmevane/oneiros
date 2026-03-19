use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SearchResult {
    pub kind: Label,
    pub id: String,
    pub content: Content,
    pub rank: f64,
}
