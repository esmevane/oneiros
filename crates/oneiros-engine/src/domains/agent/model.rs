use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Agent {
    #[builder(default)]
    pub id: AgentId,
    #[builder(into)]
    pub name: AgentName,
    #[builder(into)]
    pub persona: PersonaName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

resource_id!(AgentId);
resource_name!(AgentName);
