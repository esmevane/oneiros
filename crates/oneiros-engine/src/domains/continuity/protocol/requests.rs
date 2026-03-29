use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContinuityRequest {
    Dream { agent: AgentName },
    Introspect { agent: AgentName },
    Reflect { agent: AgentName },
    Sense { agent: AgentName, content: Content },
    Sleep { agent: AgentName },
}
