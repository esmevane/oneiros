use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectMemoryById {
    pub id: MemoryId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMemoriesFilter {
    #[serde(default)]
    pub agent: Option<AgentName>,
    #[serde(default)]
    pub level: Option<LevelName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryEvents {
    MemoryAdded(Memory),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemoryRequest {
    pub agent: AgentName,
    pub level: LevelName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryRequests {
    AddMemory(AddMemoryRequest),
    GetMemory(SelectMemoryById),
    ListMemories(ListMemoriesFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryResponses {
    MemoryAdded(Memory),
    MemoryFound(Memory),
    MemoriesListed(Vec<Memory>),
}
