use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type PersonaRecord = HasDescription<HasPrompt<Persona>>;

impl PersonaRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        persona: impl Into<Persona>,
    ) -> Self {
        HasDescription::new(
            description.into(),
            HasPrompt::new(prompt.into(), persona.into()),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Persona {
    pub name: PersonaName,
}

impl Persona {
    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> PersonaRecord {
        PersonaRecord::init(
            Description::new(description),
            Prompt::new(prompt),
            Persona {
                name: PersonaName::new(name),
            },
        )
    }
}

impl<GivenName> From<GivenName> for Persona
where
    GivenName: AsRef<str>,
{
    fn from(name: GivenName) -> Self {
        Persona {
            name: PersonaName::new(name),
        }
    }
}

domain_link!(Persona => PersonaLink);
domain_name!(PersonaName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persona_identity() {
        let primary = Persona {
            name: PersonaName::new("process"),
        };

        let other = Persona {
            name: PersonaName::new("process"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
