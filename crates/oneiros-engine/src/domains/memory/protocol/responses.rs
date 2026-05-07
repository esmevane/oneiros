use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = MemoryResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum MemoryResponse {
    MemoryAdded(MemoryAddedResponse),
    MemoryDetails(MemoryDetailsResponse),
    Memories(MemoriesResponse),
    NoMemories,
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum MemoryAddedResponse {
        V1 => { #[serde(flatten)] pub(crate) memory: Memory }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum MemoryDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) memory: Memory }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum MemoriesResponse {
        V1 => {
            pub(crate) items: Vec<Memory>,
            pub(crate) total: usize,
        }
    }
}
