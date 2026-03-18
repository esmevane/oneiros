use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageRequest {
    Upload {
        name: String,
        content_type: String,
        data: Vec<u8>,
    },
    Get {
        id: String,
    },
    List,
    Remove {
        id: String,
    },
}
