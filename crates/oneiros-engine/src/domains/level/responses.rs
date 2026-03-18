use serde::{Deserialize, Serialize};

use super::model::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelResponse {
    Set(Level),
    Found(Level),
    Listed(Vec<Level>),
    Removed,
}
