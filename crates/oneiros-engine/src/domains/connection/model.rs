use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_entity: String,
    pub to_entity: String,
    pub nature: String,
    pub description: String,
    pub created_at: String,
}
