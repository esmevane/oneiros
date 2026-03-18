use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Brain {
    pub name: String,
    pub created_at: String,
}
