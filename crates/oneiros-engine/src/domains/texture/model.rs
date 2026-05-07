use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Texture {
    #[builder(into)]
    pub(crate) name: TextureName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(into)]
    pub(crate) prompt: Prompt,
}

#[derive(Clone, Default)]
pub(crate) struct Textures(HashMap<String, Texture>);

impl Textures {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, name: &TextureName) -> Option<&Texture> {
        self.0.get(&name.to_string())
    }

    pub(crate) fn set(&mut self, texture: &Texture) -> Option<Texture> {
        self.0.insert(texture.name.to_string(), texture.clone())
    }

    pub(crate) fn remove(&mut self, name: &TextureName) -> Option<Texture> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(TextureName);
