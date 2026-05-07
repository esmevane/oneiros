use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = PersonaResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum PersonaResponse {
    PersonaSet(PersonaSetResponse),
    PersonaDetails(PersonaDetailsResponse),
    Personas(PersonasResponse),
    NoPersonas,
    PersonaRemoved(PersonaRemovedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PersonaSetResponse {
        V1 => { #[serde(flatten)] pub(crate) persona: Persona }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PersonaDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) persona: Persona }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PersonasResponse {
        V1 => {
            pub(crate) items: Vec<Persona>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum PersonaRemovedResponse {
        V1 => {
            #[builder(into)] pub(crate) name: PersonaName,
        }
    }
}
