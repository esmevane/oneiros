use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = StorageResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StorageResponse {
    StorageSet(StorageSetResponse),
    StorageDetails(StorageDetailsResponse),
    Entries(StorageEntriesResponse),
    NoEntries,
    StorageRemoved(StorageRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StorageSetResponse {
        V1 => { #[serde(flatten)] pub entry: StorageEntry }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StorageDetailsResponse {
        V1 => { #[serde(flatten)] pub entry: StorageEntry }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StorageEntriesResponse {
        V1 => {
            pub items: Vec<StorageEntry>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StorageRemovedResponse {
        V1 => {
            pub key: StorageKey,
        }
    }
}
