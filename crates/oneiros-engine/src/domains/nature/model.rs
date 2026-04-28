use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Nature {
    #[builder(into)]
    pub name: NatureName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

#[derive(Clone, Default)]
pub struct Natures(HashMap<String, Nature>);

impl Natures {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, name: &NatureName) -> Option<&Nature> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, nature: &Nature) -> Option<Nature> {
        self.0.insert(nature.name.to_string(), nature.clone())
    }

    pub fn remove(&mut self, name: &NatureName) -> Option<Nature> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(NatureName);
