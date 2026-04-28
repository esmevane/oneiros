use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = NatureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum NatureResponse {
    NatureSet(NatureSetResponse),
    NatureDetails(NatureDetailsResponse),
    Natures(NaturesResponse),
    NoNatures,
    NatureRemoved(NatureRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum NatureSetResponse {
        V1 => { #[serde(flatten)] pub nature: Nature }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum NatureDetailsResponse {
        V1 => { #[serde(flatten)] pub nature: Nature }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum NaturesResponse {
        V1 => {
            pub items: Vec<Nature>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum NatureRemovedResponse {
        V1 => {
            #[builder(into)] pub name: NatureName,
        }
    }
}
