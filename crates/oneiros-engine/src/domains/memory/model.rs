use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: String,
    pub level: String,
    pub content: String,
    pub created_at: String,
}

resource_id!(MemoryId);
