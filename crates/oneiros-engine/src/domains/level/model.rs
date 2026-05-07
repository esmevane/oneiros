use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Level {
    #[builder(into)]
    pub(crate) name: LevelName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default)]
pub(crate) struct Levels(HashMap<String, Level>);

impl Levels {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, name: &LevelName) -> Option<&Level> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, level: &Level) -> Option<Level> {
        self.0.insert(level.name.to_string(), level.clone())
    }

    pub(crate) fn remove(&mut self, name: &LevelName) -> Option<Level> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(LevelName);
