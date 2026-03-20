use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionRequest {
    Create {
        from_ref: String,
        to_ref: String,
        nature: String,
    },
    Get {
        id: String,
    },
    List {
        entity: Option<String>,
    },
    Remove {
        id: String,
    },
}
