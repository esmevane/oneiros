use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nature {
    pub name: NatureName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Nature {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenName, GivenDescription, GivenPrompt> From<(GivenName, GivenDescription, GivenPrompt)>
    for Nature
where
    GivenName: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenPrompt: AsRef<str>,
{
    fn from((name, description, prompt): (GivenName, GivenDescription, GivenPrompt)) -> Self {
        Nature {
            name: NatureName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Addressable for Nature {
    fn address_label() -> &'static str {
        "nature"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_name!(NatureName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nature_identity() {
        let primary = Nature {
            name: NatureName::new("origin"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Nature {
            name: NatureName::new("origin"),
            description: Description::new("updated"),
            prompt: Prompt::new("updated"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
