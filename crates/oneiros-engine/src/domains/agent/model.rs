use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Agent {
    pub id: AgentId,
    pub name: AgentName,
    pub persona: String,
    pub description: String,
    pub prompt: String,
}

resource_id!(AgentId);
resource_name!(AgentName);
