use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ActorResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActorResponse {
    Created(ActorCreatedResponse),
    Found(ActorFoundResponse),
    Listed(ActorsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ActorCreatedResponse {
        V1 => { #[serde(flatten)] pub actor: Actor }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ActorFoundResponse {
        V1 => { #[serde(flatten)] pub actor: Actor }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ActorsResponse {
        V1 => {
            pub items: Vec<Actor>,
            pub total: usize,
        }
    }
}
