use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub kind: String,
    pub id: String,
    pub content: String,
    pub rank: f64,
}
