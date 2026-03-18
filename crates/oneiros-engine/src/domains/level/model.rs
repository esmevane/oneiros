use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Level {
    pub name: String,
    pub description: String,
    pub prompt: String,
}
