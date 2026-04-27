use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Urge {
    Current(UrgeV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct UrgeV1 {
    #[builder(into)]
    pub name: UrgeName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Urge {
    pub fn build_v1() -> UrgeV1Builder {
        UrgeV1::builder()
    }

    pub fn name(&self) -> &UrgeName {
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
        self.0.insert(urge.name().to_string(), urge.clone())
    }

    pub fn remove(&mut self, name: &UrgeName) -> Option<Urge> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(UrgeName);
