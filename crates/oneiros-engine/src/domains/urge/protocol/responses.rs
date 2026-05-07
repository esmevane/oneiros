use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = UrgeResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum UrgeResponse {
    UrgeSet(UrgeSetResponse),
    UrgeDetails(UrgeDetailsResponse),
    Urges(UrgesResponse),
    NoUrges,
    UrgeRemoved(UrgeRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UrgeSetResponse {
        V1 => { #[serde(flatten)] pub(crate) urge: Urge }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UrgeDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) urge: Urge }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UrgesResponse {
        V1 => {
            pub(crate) items: Vec<Urge>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UrgeRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: UrgeName,
        }
    }
}
