use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Persona {
    pub name: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Persona {
    pub fn init(
        name: impl Into<PersonaName>,
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
    ) -> Persona {
        Persona {
            name: PersonaName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

domain_name!(PersonaName);
