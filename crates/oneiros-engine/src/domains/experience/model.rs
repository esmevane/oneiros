use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Experience {
    pub id: String,
    pub agent_id: String,
    pub sensation: String,
    pub description: String,
    pub created_at: String,
}
