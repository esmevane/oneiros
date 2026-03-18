use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageEntry {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub size: u64,
    pub created_at: String,
}
