use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TextureEventsType, display = "kebab-case")]
pub enum TextureEvents {
    TextureSet(TextureSet),
    TextureRemoved(TextureRemoved),
}

versioned! {
    pub enum TextureSet {
        V1 => {
            #[serde(flatten)] pub texture: Texture,
        }
    }
}

versioned! {
    pub enum TextureRemoved {
        V1 => {
            #[builder(into)] pub name: TextureName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_texture() -> Texture {
        Texture::builder()
            .name("observation")
            .description("Noticing patterns")
            .prompt("Use when you see something interesting.")
            .build()
    }

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

    #[test]
    fn texture_set_wire_format_is_flat() {
        let event = TextureEvents::TextureSet(TextureSet::V1(TextureSetV1 {
            texture: sample_texture(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "texture-set");
        assert!(
            json["data"].get("texture").is_none(),
            "flatten must elide the texture envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "observation");
        assert_eq!(json["data"]["description"], "Noticing patterns");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn texture_removed_round_trips_through_v1_layer() {
        let original = TextureEvents::TextureRemoved(TextureRemoved::V1(TextureRemovedV1 {
            name: TextureName::new("observation"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: TextureEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            TextureEvents::TextureRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "observation");
            }
            _ => panic!("wrong variant"),
        }
    }
}
