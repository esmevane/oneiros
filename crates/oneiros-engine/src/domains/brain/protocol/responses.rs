use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = BrainResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum BrainResponse {
    Created(BrainCreatedResponse),
    Found(BrainFoundResponse),
    Listed(BrainsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BrainCreatedResponse {
        V1 => { #[serde(flatten)] pub brain: Brain }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BrainFoundResponse {
        V1 => { #[serde(flatten)] pub brain: Brain }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum BrainsResponse {
        V1 => {
            pub items: Vec<Brain>,
            pub total: usize,
        }
    }
}
