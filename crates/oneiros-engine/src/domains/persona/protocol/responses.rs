use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = PersonaResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaResponse {
    PersonaSet(PersonaSetResponse),
    PersonaDetails(PersonaDetailsResponse),
    Personas(PersonasResponse),
    NoPersonas,
    PersonaRemoved(PersonaRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PersonaSetResponse {
        V1 => { #[serde(flatten)] pub persona: Persona }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PersonaDetailsResponse {
        V1 => { #[serde(flatten)] pub persona: Persona }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PersonasResponse {
        V1 => {
            pub items: Vec<Persona>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum PersonaRemovedResponse {
        V1 => {
            #[builder(into)] pub name: PersonaName,
        }
    }
}
