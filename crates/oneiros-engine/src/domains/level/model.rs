use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Level {
    Current(LevelV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LevelV1 {
    #[builder(into)]
    pub name: LevelName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Level {
    pub fn build_v1() -> LevelV1Builder {
        LevelV1::builder()
    }

    pub fn name(&self) -> &LevelName {
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
pub struct Levels(HashMap<String, Level>);

impl Levels {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, name: &LevelName) -> Option<&Level> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, level: &Level) -> Option<Level> {
        self.0.insert(level.name().to_string(), level.clone())
    }

    pub fn remove(&mut self, name: &LevelName) -> Option<Level> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(LevelName);
