use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectPersonaByName {
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved(SelectPersonaByName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaRequests {
    SetPersona(Persona),
    RemovePersona(SelectPersonaByName),
    GetPersona(SelectPersonaByName),
    ListPersonas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaResponses {
    PersonaSet(Persona),
    PersonaFound(Persona),
    PersonasListed(Vec<Persona>),
    PersonaRemoved,
}
