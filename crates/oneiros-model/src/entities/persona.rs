use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Persona {
    pub name: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Persona {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenName, GivenDescription, GivenPrompt> From<(GivenName, GivenDescription, GivenPrompt)>
    for Persona
where
    GivenName: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenPrompt: AsRef<str>,
{
    fn from((name, description, prompt): (GivenName, GivenDescription, GivenPrompt)) -> Self {
        Persona {
            name: PersonaName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Addressable for Persona {
    fn address_label() -> &'static str {
        "persona"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_name!(PersonaName);
oneiros_link::domain_link!(PersonaLink, "persona");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persona_identity() {
        let primary = Persona {
            name: PersonaName::new("process"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Persona {
            name: PersonaName::new("process"),
            description: Description::new("updated"),
            prompt: Prompt::new("updated"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
