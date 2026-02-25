use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Texture {
    pub name: TextureName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Texture {
    pub fn init(
        name: impl Into<TextureName>,
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            prompt: prompt.into(),
        }
    }

    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> Texture {
        Texture {
            name: TextureName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

domain_name!(TextureName);
