use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ActorResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ActorResponse {
    Created(ActorCreatedResponse),
    Found(ActorFoundResponse),
    Listed(ActorsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ActorCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) actor: Actor }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ActorFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) actor: Actor }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ActorsResponse {
        V1 => {
            pub(crate) items: Vec<Actor>,
            pub(crate) total: usize,
        }
    }
}
