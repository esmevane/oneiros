use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: AgentName,
    pub persona: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

domain_id!(AgentId);
domain_name!(AgentName);
