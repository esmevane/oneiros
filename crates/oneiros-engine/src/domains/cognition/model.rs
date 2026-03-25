use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Cognition {
    #[builder(default)]
    pub id: CognitionId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub texture: TextureName,
    #[builder(into)]
    pub content: Content,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(CognitionId);
