use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type LevelRecord = HasDescription<HasPrompt<Level>>;

impl LevelRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        level: impl Into<Level>,
    ) -> Self {
        HasDescription::new(
            description.into(),
            HasPrompt::new(prompt.into(), level.into()),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Level {
    pub name: LevelName,
}

impl Level {
    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> LevelRecord {
        LevelRecord::init(
            Description::new(description),
            Prompt::new(prompt),
            Level {
                name: LevelName::new(name),
            },
        )
    }
}

impl<GivenName> From<GivenName> for Level
where
    GivenName: AsRef<str>,
{
    fn from(name: GivenName) -> Self {
        Level {
            name: LevelName::new(name),
        }
    }
}

domain_link!(Level => LevelLink);
domain_name!(LevelName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_identity() {
        let primary = Level {
            name: LevelName::new("project"),
        };

        let other = Level {
            name: LevelName::new("project"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
