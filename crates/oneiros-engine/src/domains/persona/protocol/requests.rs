use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetPersona {
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemovePersona {
    pub name: PersonaName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PersonaRequest {
    Set(Persona),
    Get(GetPersona),
    List,
    Remove(RemovePersona),
}
