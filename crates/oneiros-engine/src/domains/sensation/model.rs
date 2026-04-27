use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Sensation {
    Current(SensationV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SensationV1 {
    #[builder(into)]
    pub name: SensationName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Sensation {
    pub fn build_v1() -> SensationV1Builder {
        SensationV1::builder()
    }

    pub fn name(&self) -> &SensationName {
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
        self.0
            .insert(sensation.name().to_string(), sensation.clone())
    }

    pub fn remove(&mut self, name: &SensationName) -> Option<Sensation> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(SensationName);
