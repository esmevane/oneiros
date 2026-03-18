use serde::{Deserialize, Serialize};

use super::model::Persona;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaResponse {
    Set(Persona),
    Found(Persona),
    Listed(Vec<Persona>),
    Removed,
}
