use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaResponse {
    PersonaSet(PersonaName),
    PersonaDetails(Persona),
    Personas(Listed<Persona>),
    NoPersonas,
    PersonaRemoved(PersonaName),
}
