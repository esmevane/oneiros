use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum MemoryRequest {
    Add {
        agent: AgentName,
        level: LevelName,
        content: Content,
    },
    Get {
        id: MemoryId,
    },
    List {
        agent: Option<AgentName>,
    },
}
