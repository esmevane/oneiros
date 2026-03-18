use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Experience {
    pub id: ExperienceId,
    pub agent_id: String,
    pub sensation: String,
    pub description: String,
    pub created_at: String,
}

resource_id!(ExperienceId);
