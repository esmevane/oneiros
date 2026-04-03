use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelResponse {
    LevelSet(LevelName),
    LevelDetails(Response<Level>),
    Levels(Listed<Response<Level>>),
    NoLevels,
    LevelRemoved(LevelName),
}
