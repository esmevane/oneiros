use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl Indexable<TextureName> for Texture {
    fn id(&self) -> TextureName {
        self.name.clone()
    }
}

pub(crate) type Textures = EntityIndex<TextureName, Texture>;

resource_name!(TextureName);
