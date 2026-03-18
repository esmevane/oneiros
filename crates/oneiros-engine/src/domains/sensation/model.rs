use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sensation {
    pub name: String,
    pub description: String,
    pub prompt: String,
}
