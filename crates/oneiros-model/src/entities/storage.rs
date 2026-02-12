use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: StorageKey,
    pub description: Description,
    pub hash: ContentHash,
}

domain_name!(StorageKey);
