use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Sensation {
    #[builder(into)]
    pub(crate) name: SensationName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default)]
pub(crate) struct Sensations(HashMap<String, Sensation>);

impl Sensations {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, name: &SensationName) -> Option<&Sensation> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, sensation: &Sensation) -> Option<Sensation> {
        self.0.insert(sensation.name.to_string(), sensation.clone())
    }

    pub(crate) fn remove(&mut self, name: &SensationName) -> Option<Sensation> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(SensationName);
