use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum MemoryResponse {
    MemoryAdded(Response<Memory>),
    MemoryDetails(Response<Memory>),
    Memories(Listed<Response<Memory>>),
    NoMemories,
}
