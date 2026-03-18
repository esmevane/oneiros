use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: String,
}
