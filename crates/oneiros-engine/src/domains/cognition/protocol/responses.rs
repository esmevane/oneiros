use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = CognitionResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum CognitionResponse {
    CognitionAdded(CognitionAddedResponse),
    CognitionDetails(CognitionDetailsResponse),
    Cognitions(CognitionsResponse),
    NoCognitions,
}

versioned! {
    #[derive(JsonSchema)]
    pub enum CognitionAddedResponse {
        V1 => { #[serde(flatten)] pub cognition: Cognition }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum CognitionDetailsResponse {
        V1 => { #[serde(flatten)] pub cognition: Cognition }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum CognitionsResponse {
        V1 => {
            pub items: Vec<Cognition>,
            pub total: usize,
        }
    }
}
