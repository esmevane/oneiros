use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = StorageResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum StorageResponse {
    StorageSet(StorageSetResponse),
    StorageDetails(StorageDetailsResponse),
    Entries(StorageEntriesResponse),
    NoEntries,
    StorageRemoved(StorageRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StorageSetResponse {
        V1 => { #[serde(flatten)] pub(crate) entry: StorageEntry }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StorageDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) entry: StorageEntry }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StorageEntriesResponse {
        V1 => {
            pub(crate) items: Vec<StorageEntry>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StorageRemovedResponse {
        V1 => {
            pub(crate) key: StorageKey,
        }
    }
}
