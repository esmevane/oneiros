use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = PersonaResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaResponse {
    PersonaSet(PersonaName),
    PersonaDetails(Response<Persona>),
    Personas(Listed<Response<Persona>>),
    NoPersonas,
    PersonaRemoved(PersonaName),
}
