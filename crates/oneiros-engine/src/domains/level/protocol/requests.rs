use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelRequest {
    Set(Level),
    Get { name: LevelName },
    List,
    Remove { name: LevelName },
}
