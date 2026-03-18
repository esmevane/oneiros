use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: String,
}
