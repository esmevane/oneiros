use serde::{Deserialize, Serialize};

use super::model::Persona;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaRequest {
    Set(Persona),
    Get { name: String },
    List,
    Remove { name: String },
}
