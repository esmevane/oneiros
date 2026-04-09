use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = LevelResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelResponse {
    LevelSet(LevelName),
    LevelDetails(Response<Level>),
    Levels(Listed<Response<Level>>),
    NoLevels,
    LevelRemoved(LevelName),
}
