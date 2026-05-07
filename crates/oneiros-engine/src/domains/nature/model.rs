use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Nature {
    #[builder(into)]
    pub(crate) name: NatureName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default)]
pub(crate) struct Natures(HashMap<String, Nature>);

impl Natures {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, name: &NatureName) -> Option<&Nature> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, nature: &Nature) -> Option<Nature> {
        self.0.insert(nature.name.to_string(), nature.clone())
    }

    pub(crate) fn remove(&mut self, name: &NatureName) -> Option<Nature> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(NatureName);
