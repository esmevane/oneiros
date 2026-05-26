use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TrailResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum TrailResponse {
    TrailEvents(TrailEventsResponse),
    EmittedRefs(EmittedRefsResponse),
    NoTrail,
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TrailEventsResponse {
        V1 => {
            pub(crate) items: Vec<TrailEntry>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum EmittedRefsResponse {
        V1 => {
            pub(crate) items: Vec<RefToken>,
            pub(crate) total: usize,
        }
    }
}
