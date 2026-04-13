use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = MemoryResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum MemoryResponse {
    MemoryAdded(Response<Memory>),
    MemoryDetails(Response<Memory>),
    Memories(Listed<Response<Memory>>),
    NoMemories,
}
