use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Level {
    pub name: LevelName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Level {
    pub fn construct_from_db(row: impl Into<Self>) -> Self {
        row.into()
    }
}

impl<GivenName, GivenDescription, GivenPrompt> From<(GivenName, GivenDescription, GivenPrompt)>
    for Level
where
    GivenName: AsRef<str>,
    GivenDescription: AsRef<str>,
    GivenPrompt: AsRef<str>,
{
    fn from((name, description, prompt): (GivenName, GivenDescription, GivenPrompt)) -> Self {
        Level {
            name: LevelName::new(name),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        }
    }
}

impl Addressable for Level {
    fn address_label() -> &'static str {
        "level"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_name!(LevelName);
oneiros_link::domain_link!(LevelLink, "level");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_identity() {
        let primary = Level {
            name: LevelName::new("project"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Level {
            name: LevelName::new("project"),
            description: Description::new("updated"),
            prompt: Prompt::new("updated"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
