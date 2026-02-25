use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Level {
    pub name: LevelName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Level {
    pub fn init(
        name: impl Into<LevelName>,
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
    ) -> Level {
        Level {
            name: LevelName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

domain_name!(LevelName);
