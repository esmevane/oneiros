use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct AddCognition {
    pub agent: AgentName,
    pub texture: TextureName,
    pub content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetCognition {
    pub id: CognitionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListCognitions {
    #[arg(long)]
    pub agent: Option<AgentName>,
    #[arg(long)]
    pub texture: Option<TextureName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionRequest {
    Add(AddCognition),
    Get(GetCognition),
    List(ListCognitions),
}
