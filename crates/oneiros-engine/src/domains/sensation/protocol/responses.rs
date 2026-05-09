use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = SensationResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum SensationResponse {
    SensationSet(SensationSetResponse),
    SensationDetails(SensationDetailsResponse),
    Sensations(SensationsResponse),
    NoSensations,
    SensationRemoved(SensationRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SensationSetResponse {
        V1 => { #[serde(flatten)] pub(crate) sensation: Sensation }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SensationDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) sensation: Sensation }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SensationsResponse {
        V1 => {
            pub(crate) items: Vec<Sensation>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SensationRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: SensationName,
        }
    }
}
