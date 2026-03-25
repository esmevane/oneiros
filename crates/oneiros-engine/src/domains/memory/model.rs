use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Memory {
    #[builder(default)]
    pub id: MemoryId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub level: LevelName,
    #[builder(into)]
    pub content: Content,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(MemoryId);
