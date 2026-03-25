use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionRequest {
    Add {
        agent: AgentName,
        texture: TextureName,
        content: Content,
    },
    Get {
        id: CognitionId,
    },
    List {
        agent: Option<AgentName>,
        texture: Option<TextureName>,
    },
}
