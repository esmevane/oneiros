use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sensation {
    pub name: SensationName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Sensation {
    pub fn init(
        name: impl Into<SensationName>,
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
    ) -> Sensation {
        Sensation {
            name: SensationName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

domain_name!(SensationName);
