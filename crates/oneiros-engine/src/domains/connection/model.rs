use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Connection {
    pub id: ConnectionId,
    pub from_entity: String,
    pub to_entity: String,
    pub nature: NatureName,
    pub description: Description,
    pub created_at: Timestamp,
}

resource_id!(ConnectionId);
