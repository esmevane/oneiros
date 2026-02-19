use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Texture {
    pub name: TextureName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Texture {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenName, GivenDescription, GivenPrompt> From<(GivenName, GivenDescription, GivenPrompt)>
    for Texture
where
    GivenName: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenPrompt: AsRef<str>,
{
    fn from((name, description, prompt): (GivenName, GivenDescription, GivenPrompt)) -> Self {
        Texture {
            name: TextureName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Addressable for Texture {
    fn address_label() -> &'static str {
        "texture"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_name!(TextureName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_identity() {
        let primary = Texture {
            name: TextureName::new("observation"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Texture {
            name: TextureName::new("observation"),
            description: Description::new("updated"),
            prompt: Prompt::new("updated"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
