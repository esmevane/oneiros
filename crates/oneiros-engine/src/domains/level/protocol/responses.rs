use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = LevelResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum LevelResponse {
    LevelSet(LevelSetResponse),
    LevelDetails(LevelDetailsResponse),
    Levels(LevelsResponse),
    NoLevels,
    LevelRemoved(LevelRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum LevelSetResponse {
        V1 => { #[serde(flatten)] pub(crate) level: Level }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum LevelDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) level: Level }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum LevelsResponse {
        V1 => {
            pub(crate) items: Vec<Level>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum LevelRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: LevelName,
        }
    }
}
