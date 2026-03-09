use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectPersonaByName {
    pub name: PersonaName,
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonaRequest {
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePersonaRequest {
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPersonasRequest;

// ── Protocol enums ─────────────────────────────────────────────────

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
    RemovePersona(RemovePersonaRequest),
    GetPersona(GetPersonaRequest),
    ListPersonas(ListPersonasRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaResponses {
    PersonaSet(Persona),
    PersonaFound(Persona),
    PersonasListed(Vec<Persona>),
    PersonaRemoved,
}
