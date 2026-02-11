use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Persona {
    pub name: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

domain_name!(PersonaName);
