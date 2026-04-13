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
pub(crate) struct Persona {
    #[builder(into)]
    pub(crate) name: PersonaName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub(crate) description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "personas")]
pub(crate) struct Personas(HashMap<String, Persona>);

impl Personas {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, name: &PersonaName) -> Option<&Persona> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, persona: &Persona) -> Option<Persona> {
        self.0.insert(persona.name.to_string(), persona.clone())
    }

    pub(crate) fn remove(&mut self, name: &PersonaName) -> Option<Persona> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(PersonaName);
