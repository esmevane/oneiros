use bon::Builder;
use clap::Args;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Args, Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Persona {
    #[builder(into)]
    pub name: PersonaName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "personas")]
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
        self.0.insert(persona.name.to_string(), persona.clone())
    }

    pub fn remove(&mut self, name: &PersonaName) -> Option<Persona> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(PersonaName);
