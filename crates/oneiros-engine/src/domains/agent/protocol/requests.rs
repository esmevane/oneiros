use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateAgent {
    pub name: AgentName,
    pub persona: PersonaName,
    #[arg(long, default_value = "")]
    pub description: Description,
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetAgent {
    pub name: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveAgent {
    pub name: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateAgent {
    pub name: AgentName,
    pub persona: PersonaName,
    #[arg(long, default_value = "")]
    pub description: Description,
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AgentRequest {
    Create(CreateAgent),
    Get(GetAgent),
    List,
    Update(UpdateAgent),
    Remove(RemoveAgent),
}
