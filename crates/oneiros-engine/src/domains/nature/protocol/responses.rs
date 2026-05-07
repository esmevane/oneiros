use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = NatureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum NatureResponse {
    NatureSet(NatureSetResponse),
    NatureDetails(NatureDetailsResponse),
    Natures(NaturesResponse),
    NoNatures,
    NatureRemoved(NatureRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum NatureSetResponse {
        V1 => { #[serde(flatten)] pub(crate) nature: Nature }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum NatureDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) nature: Nature }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum NaturesResponse {
        V1 => {
            pub(crate) items: Vec<Nature>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum NatureRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: NatureName,
        }
    }
}
