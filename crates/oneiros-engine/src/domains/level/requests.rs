use serde::{Deserialize, Serialize};

use super::model::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelRequest {
    Set(Level),
    Get { name: String },
    List,
    Remove { name: String },
}
