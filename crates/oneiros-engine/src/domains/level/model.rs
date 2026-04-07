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
pub struct Level {
    #[builder(into)]
    pub name: LevelName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "levels")]
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
        self.0.insert(level.name.to_string(), level.clone())
    }

    pub fn remove(&mut self, name: &LevelName) -> Option<Level> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(LevelName);
