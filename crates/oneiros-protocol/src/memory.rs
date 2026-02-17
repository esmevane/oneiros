use oneiros_model::{AgentName, Content, LevelName, Memory};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
}
