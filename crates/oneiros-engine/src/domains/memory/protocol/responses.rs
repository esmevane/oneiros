use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = MemoryResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum MemoryResponse {
    MemoryAdded(MemoryAddedResponse),
    MemoryDetails(MemoryDetailsResponse),
    Memories(MemoriesResponse),
    NoMemories,
}

versioned! {
    #[derive(JsonSchema)]
    pub enum MemoryAddedResponse {
        V1 => { #[serde(flatten)] pub memory: Memory }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum MemoryDetailsResponse {
        V1 => { #[serde(flatten)] pub memory: Memory }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum MemoriesResponse {
        V1 => {
            pub items: Vec<Memory>,
            pub total: usize,
        }
    }
}
