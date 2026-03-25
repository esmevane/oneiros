use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateConnection {
    pub nature: NatureName,
    pub from_ref: RefToken,
    pub to_ref: RefToken,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetConnection {
    pub id: ConnectionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListConnections {
    #[arg(long)]
    pub entity: Option<RefToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveConnection {
    pub id: ConnectionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionRequest {
    Create(CreateConnection),
    Get(GetConnection),
    List(ListConnections),
    Remove(RemoveConnection),
}
