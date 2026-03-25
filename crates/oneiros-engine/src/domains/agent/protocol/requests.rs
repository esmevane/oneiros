use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentRequest {
    Create {
        name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    },
    Get {
        name: AgentName,
    },
    List,
    Update {
        name: AgentName,
        persona: PersonaName,
        description: Description,
        prompt: Prompt,
    },
    Remove {
        name: AgentName,
    },
}
