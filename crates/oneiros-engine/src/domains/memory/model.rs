use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: AgentName,
    pub level: LevelName,
    pub content: Content,
    pub created_at: Timestamp,
}

resource_id!(MemoryId);
