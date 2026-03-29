use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceRequest {
    Create {
        agent: AgentName,
        sensation: SensationName,
        description: Description,
    },
    Get {
        id: ExperienceId,
    },
    List {
        agent: Option<AgentName>,
    },
    UpdateDescription {
        id: ExperienceId,
        description: Description,
    },
    UpdateSensation {
        id: ExperienceId,
        sensation: SensationName,
    },
}
