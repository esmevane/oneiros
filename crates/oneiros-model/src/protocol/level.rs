use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectLevelByName {
    pub name: LevelName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(Level),
    LevelRemoved(SelectLevelByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelRequests {
    SetLevel(Level),
    RemoveLevel(SelectLevelByName),
    GetLevel(SelectLevelByName),
    ListLevels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelResponses {
    LevelSet(Level),
    LevelFound(Level),
    LevelsListed(Vec<Level>),
    LevelRemoved,
}
