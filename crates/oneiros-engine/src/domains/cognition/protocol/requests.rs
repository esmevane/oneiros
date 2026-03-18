use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionRequest {
    Add {
        agent: String,
        texture: String,
        content: String,
    },
    Get {
        id: String,
    },
    List {
        agent: Option<String>,
        texture: Option<String>,
    },
}
