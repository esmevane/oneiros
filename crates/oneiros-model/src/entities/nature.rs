use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type NatureRecord = HasDescription<HasPrompt<Nature>>;

impl NatureRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        nature: impl Into<Nature>,
    ) -> Self {
        HasDescription::new(
            description.into(),
            HasPrompt::new(prompt.into(), nature.into()),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Nature {
    pub name: NatureName,
}

impl Nature {
    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> NatureRecord {
        NatureRecord::init(
            Description::new(description),
            Prompt::new(prompt),
            Nature {
                name: NatureName::new(name),
            },
        )
    }
}

impl<GivenName> From<GivenName> for Nature
where
    GivenName: AsRef<str>,
{
    fn from(name: GivenName) -> Self {
        Nature {
            name: NatureName::new(name),
        }
    }
}

domain_link!(Nature => NatureLink);
domain_name!(NatureName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nature_identity() {
        let primary = Nature {
            name: NatureName::new("origin"),
        };

        let other = Nature {
            name: NatureName::new("origin"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
