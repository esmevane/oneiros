use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sensation {
    pub name: SensationName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Sensation {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenName, GivenDescription, GivenPrompt> From<(GivenName, GivenDescription, GivenPrompt)>
    for Sensation
where
    GivenName: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenPrompt: AsRef<str>,
{
    fn from((name, description, prompt): (GivenName, GivenDescription, GivenPrompt)) -> Self {
        Sensation {
            name: SensationName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Addressable for Sensation {
    fn address_label() -> &'static str {
        "sensation"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_name!(SensationName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensation_identity() {
        let primary = Sensation {
            name: SensationName::new("echoes"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Sensation {
            name: SensationName::new("echoes"),
            description: Description::new("updated"),
            prompt: Prompt::new("updated"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
