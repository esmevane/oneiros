use serde::{Deserialize, Serialize};

use super::model::Persona;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved(PersonaRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaRemoved {
    pub name: String,
}
