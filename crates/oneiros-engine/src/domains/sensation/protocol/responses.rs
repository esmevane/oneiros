use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = SensationResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SensationResponse {
    SensationSet(SensationSetResponse),
    SensationDetails(SensationDetailsResponse),
    Sensations(SensationsResponse),
    NoSensations,
    SensationRemoved(SensationRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SensationSetResponse {
        V1 => { #[serde(flatten)] pub sensation: Sensation }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SensationDetailsResponse {
        V1 => { #[serde(flatten)] pub sensation: Sensation }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SensationsResponse {
        V1 => {
            pub items: Vec<Sensation>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SensationRemovedResponse {
        V1 => {
            #[builder(into)] pub name: SensationName,
        }
    }
}
