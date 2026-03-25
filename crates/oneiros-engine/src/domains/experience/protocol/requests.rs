use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceRequest {
    Create {
        agent: String,
        sensation: String,
        description: String,
    },
    Get {
        id: String,
    },
    List {
        agent: Option<String>,
    },
    UpdateDescription {
        id: String,
        description: String,
    },
    UpdateSensation {
        id: String,
        sensation: String,
    },
}
