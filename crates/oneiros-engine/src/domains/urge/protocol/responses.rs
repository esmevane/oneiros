use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = UrgeResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UrgeResponse {
    UrgeSet(UrgeSetResponse),
    UrgeDetails(UrgeDetailsResponse),
    Urges(UrgesResponse),
    NoUrges,
    UrgeRemoved(UrgeRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UrgeSetResponse {
        V1 => { #[serde(flatten)] pub urge: Urge }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UrgeDetailsResponse {
        V1 => { #[serde(flatten)] pub urge: Urge }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UrgesResponse {
        V1 => {
            pub items: Vec<Urge>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UrgeRemovedResponse {
        V1 => {
            #[builder(into)] pub name: UrgeName,
        }
    }
}
