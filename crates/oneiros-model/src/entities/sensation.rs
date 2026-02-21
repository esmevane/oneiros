use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type SensationRecord = HasDescription<HasPrompt<Sensation>>;

impl SensationRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        sensation: impl Into<Sensation>,
    ) -> Self {
        HasDescription::new(
            description.into(),
            HasPrompt::new(prompt.into(), sensation.into()),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sensation {
    pub name: SensationName,
}

impl Sensation {
    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> SensationRecord {
        SensationRecord::init(
            Description::new(description),
            Prompt::new(prompt),
            Sensation {
                name: SensationName::new(name),
            },
        )
    }
}

impl<GivenName> From<GivenName> for Sensation
where
    GivenName: AsRef<str>,
{
    fn from(name: GivenName) -> Self {
        Sensation {
            name: SensationName::new(name),
        }
    }
}

domain_link!(Sensation => SensationLink);
domain_name!(SensationName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensation_identity() {
        let primary = Sensation {
            name: SensationName::new("echoes"),
        };

        let other = Sensation {
            name: SensationName::new("echoes"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
