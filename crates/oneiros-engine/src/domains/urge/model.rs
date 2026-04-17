use bon::Builder;
use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Args, Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Urge {
    #[builder(into)]
    pub name: UrgeName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Clone, Default)]
pub struct Urges(HashMap<String, Urge>);

impl Urges {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values(&self) -> impl Iterator<Item = &Urge> {
        self.0.values()
    }

    pub fn get(&self, name: &UrgeName) -> Option<&Urge> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, urge: &Urge) -> Option<Urge> {
        self.0.insert(urge.name.to_string(), urge.clone())
    }

    pub fn remove(&mut self, name: &UrgeName) -> Option<Urge> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(UrgeName);
