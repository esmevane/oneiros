use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Experience {
    #[builder(default)]
    pub id: ExperienceId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub sensation: SensationName,
    #[builder(into)]
    pub description: Description,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

resource_id!(ExperienceId);
