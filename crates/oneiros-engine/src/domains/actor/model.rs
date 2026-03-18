use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: String,
}
