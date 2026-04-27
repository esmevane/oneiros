use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

/// Versioned domain entity for a cognitive texture vocabulary entry.
///
/// `Current` is the version users construct via `Texture::build_v1()`.
/// Older variants (V0, V1, ...) are added below `Current` as fallbacks
/// when shape evolution happens; `#[serde(untagged)]` chooses the right
/// variant by matching JSON shape during deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Texture {
    Current(TextureV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct TextureV1 {
    #[builder(into)]
    pub name: TextureName,
    #[builder(into)]
    pub description: Description,
    #[builder(into)]
    pub prompt: Prompt,
}

impl Texture {
    /// Open the V1 builder. Construction sites read as
    /// `Texture::Current(Texture::build_v1().name("foo").build())`,
    /// keeping the version visible at every callsite.
    pub fn build_v1() -> TextureV1Builder {
        TextureV1::builder()
    }

    pub fn name(&self) -> &TextureName {
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
        self.0.insert(texture.name().to_string(), texture.clone())
    }

    pub fn remove(&mut self, name: &TextureName) -> Option<Texture> {
        self.0.remove(&name.to_string())
    }
}

resource_name!(TextureName);
