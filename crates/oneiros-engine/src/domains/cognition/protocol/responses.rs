use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = CognitionResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum CognitionResponse {
    CognitionAdded(CognitionAddedResponse),
    CognitionDetails(CognitionDetailsResponse),
    Cognitions(CognitionsResponse),
    NoCognitions,
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CognitionAddedResponse {
        V1 => { #[serde(flatten)] pub(crate) cognition: Cognition }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CognitionDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) cognition: Cognition }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CognitionsResponse {
        V1 => {
            pub(crate) items: Vec<Cognition>,
            pub(crate) total: usize,
        }
    }
}
