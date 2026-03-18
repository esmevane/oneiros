use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    pub id: String,
    pub agent_id: String,
    pub level: String,
    pub content: String,
    pub created_at: String,
}
