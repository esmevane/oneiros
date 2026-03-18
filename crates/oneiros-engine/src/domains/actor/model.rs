use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Actor {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: String,
}
