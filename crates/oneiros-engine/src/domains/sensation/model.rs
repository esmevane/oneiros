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
pub struct Sensation {
    #[builder(into)]
    pub name: SensationName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "sensations")]
pub struct Sensations(HashMap<String, Sensation>);

impl Sensations {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, name: &SensationName) -> Option<&Sensation> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, sensation: &Sensation) -> Option<Sensation> {
        self.0.insert(sensation.name.to_string(), sensation.clone())
    }

    pub fn remove(&mut self, name: &SensationName) -> Option<Sensation> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(SensationName);
