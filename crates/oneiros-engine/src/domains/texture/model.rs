use bon::Builder;
use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Args, Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Texture {
    #[builder(into)]
    pub name: TextureName,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub description: Description,
    #[builder(into)]
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

#[derive(Clone, Default)]
pub struct Textures(HashMap<String, Texture>);

impl Textures {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, name: &TextureName) -> Option<&Texture> {
        self.0.get(&name.to_string())
    }

    pub fn set(&mut self, texture: &Texture) -> Option<Texture> {
        self.0.insert(texture.name.to_string(), texture.clone())
    }

    pub fn remove(&mut self, name: &TextureName) -> Option<Texture> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(TextureName);
