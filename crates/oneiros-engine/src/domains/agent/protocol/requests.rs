use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentRequest {
    Create {
        name: String,
        persona: String,
        description: String,
        prompt: String,
    },
    Get {
        name: String,
    },
    List,
    Update {
        name: String,
        persona: String,
        description: String,
        prompt: String,
    },
    Remove {
        name: String,
    },
}
