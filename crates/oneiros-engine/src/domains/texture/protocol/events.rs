use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TextureEventsType, display = "kebab-case")]
pub enum TextureEvents {
    TextureSet(Texture),
    TextureRemoved(TextureRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (TextureEventsType::TextureSet, "texture-set"),
            (TextureEventsType::TextureRemoved, "texture-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextureRemoved {
    Current(TextureRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct TextureRemovedV1 {
    pub name: TextureName,
}

impl TextureRemoved {
    pub fn build_v1() -> TextureRemovedV1Builder {
        TextureRemovedV1::builder()
    }

    pub fn name(&self) -> &TextureName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}
