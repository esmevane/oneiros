use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Cognition {
    pub id: CognitionId,
    pub agent_id: String,
    pub texture: String,
    pub content: String,
    pub created_at: String,
}

resource_id!(CognitionId);
