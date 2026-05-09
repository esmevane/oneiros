use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = BrainResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum BrainResponse {
    Created(BrainCreatedResponse),
    Found(BrainFoundResponse),
    Listed(BrainsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BrainCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) brain: Brain }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BrainFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) brain: Brain }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum BrainsResponse {
        V1 => {
            pub(crate) items: Vec<Brain>,
            pub(crate) total: usize,
        }
    }
}
