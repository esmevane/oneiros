use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaRequest {
    Set(Persona),
    Get { name: PersonaName },
    List,
    Remove { name: PersonaName },
}
