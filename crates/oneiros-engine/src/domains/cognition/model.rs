use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Cognition {
    pub id: CognitionId,
    pub agent_id: AgentName,
    pub texture: TextureName,
    pub content: Content,
    pub created_at: Timestamp,
}

resource_id!(CognitionId);
