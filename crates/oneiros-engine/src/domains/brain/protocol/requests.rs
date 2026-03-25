use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateBrain {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetBrain {
    pub name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BrainRequest {
    Create(CreateBrain),
    Get(GetBrain),
    List,
}
