use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Experience {
    pub id: ExperienceId,
    pub agent_id: AgentName,
    pub sensation: SensationName,
    pub description: Description,
    pub created_at: Timestamp,
}

resource_id!(ExperienceId);
