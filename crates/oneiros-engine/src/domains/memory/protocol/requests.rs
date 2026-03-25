use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct AddMemory {
    pub agent: AgentName,
    pub level: LevelName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetMemory {
    pub id: MemoryId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListMemories {
    #[arg(long)]
    pub agent: Option<AgentName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum MemoryRequest {
    Add(AddMemory),
    Get(GetMemory),
    List(ListMemories),
}
