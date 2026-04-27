use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Persona {
    Current(PersonaV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PersonaV1 {
    #[builder(into)]
    pub name: PersonaName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Persona {
    pub fn build_v1() -> PersonaV1Builder {
        PersonaV1::builder()
    }

    pub fn name(&self) -> &PersonaName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn description(&self) -> &Description {
        match self {
            Self::Current(v) => &v.description,
        }
    }

    pub fn prompt(&self) -> &Prompt {
        match self {
            Self::Current(v) => &v.prompt,
        }
    }
}

#[derive(Clone, Default)]
pub struct Personas(HashMap<String, Persona>);

impl Personas {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, name: &PersonaName) -> Option<&Persona> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, persona: &Persona) -> Option<Persona> {
        self.0.insert(persona.name().to_string(), persona.clone())
    }

    pub fn remove(&mut self, name: &PersonaName) -> Option<Persona> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(PersonaName);
