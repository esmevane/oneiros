use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved(PersonaRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaRemoved {
    pub name: PersonaName,
}
