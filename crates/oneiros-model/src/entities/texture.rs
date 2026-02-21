use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

pub type TextureRecord = HasDescription<HasPrompt<Texture>>;

impl TextureRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        texture: impl Into<Texture>,
    ) -> Self {
        HasDescription::new(
            description.into(),
            HasPrompt::new(prompt.into(), texture.into()),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Texture {
    pub name: TextureName,
}

impl Texture {
    pub fn construct_from_db(
        (name, description, prompt): (impl AsRef<str>, impl AsRef<str>, impl AsRef<str>),
    ) -> TextureRecord {
        TextureRecord::init(
            Description::new(description),
            Prompt::new(prompt),
            Texture {
                name: TextureName::new(name),
            },
        )
    }
}

impl<GivenName> From<GivenName> for Texture
where
    GivenName: AsRef<str>,
{
    fn from(name: GivenName) -> Self {
        Texture {
            name: TextureName::new(name),
        }
    }
}

domain_link!(Texture => TextureLink);
domain_name!(TextureName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_identity() {
        let primary = Texture {
            name: TextureName::new("observation"),
        };

        let other = Texture {
            name: TextureName::new("observation"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
