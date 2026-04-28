use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = LevelResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LevelResponse {
    LevelSet(LevelSetResponse),
    LevelDetails(LevelDetailsResponse),
    Levels(LevelsResponse),
    NoLevels,
    LevelRemoved(LevelRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum LevelSetResponse {
        V1 => { #[serde(flatten)] pub level: Level }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum LevelDetailsResponse {
        V1 => { #[serde(flatten)] pub level: Level }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum LevelsResponse {
        V1 => {
            pub items: Vec<Level>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum LevelRemovedResponse {
        V1 => {
            #[builder(into)] pub name: LevelName,
        }
    }
}
