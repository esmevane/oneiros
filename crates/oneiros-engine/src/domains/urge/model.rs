use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Urge {
    #[builder(into)]
    pub(crate) name: UrgeName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default)]
pub(crate) struct Urges(HashMap<String, Urge>);

impl Urges {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &Urge> {
        self.0.values()
    }

    pub(crate) fn get(&self, name: &UrgeName) -> Option<&Urge> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, urge: &Urge) -> Option<Urge> {
        self.0.insert(urge.name.to_string(), urge.clone())
    }

    pub(crate) fn remove(&mut self, name: &UrgeName) -> Option<Urge> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(UrgeName);
