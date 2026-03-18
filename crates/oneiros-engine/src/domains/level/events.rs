use serde::{Deserialize, Serialize};

use super::model::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(Level),
    LevelRemoved(LevelRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelRemoved {
    pub name: String,
}
