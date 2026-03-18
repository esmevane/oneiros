use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Connection {
    pub id: ConnectionId,
    pub from_entity: String,
    pub to_entity: String,
    pub nature: String,
    pub description: String,
    pub created_at: String,
}

resource_id!(ConnectionId);
