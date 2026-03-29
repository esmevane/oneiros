use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageRequest {
    Upload {
        key: StorageKey,
        description: Description,
        data: Vec<u8>,
    },
    Get {
        id: StorageKey,
    },
    List,
    Remove {
        id: StorageKey,
    },
}
