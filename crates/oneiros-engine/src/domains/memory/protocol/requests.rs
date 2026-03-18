use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum MemoryRequest {
    Add {
        agent: String,
        level: String,
        content: String,
    },
    Get {
        id: String,
    },
    List {
        agent: Option<String>,
    },
}
