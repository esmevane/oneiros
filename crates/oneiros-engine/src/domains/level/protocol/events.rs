use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = LevelEventsType, display = "kebab-case")]
pub enum LevelEvents {
    LevelSet(LevelSet),
    LevelRemoved(LevelRemoved),
}

versioned! {
    pub enum LevelSet {
        V1 => {
            #[serde(flatten)] pub level: Level,
        }
    }
}

versioned! {
    pub enum LevelRemoved {
        V1 => {
            #[builder(into)] pub name: LevelName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_level() -> Level {
        Level::builder()
            .name("working")
            .description("Short-term")
            .prompt("")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (LevelEventsType::LevelSet, "level-set"),
            (LevelEventsType::LevelRemoved, "level-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn level_set_wire_format_is_flat() {
        // V1 embeds `Level` with `#[serde(flatten)]`, so the wire shape stays
        // at the model fields and never gains a `level` envelope.
        let event = LevelEvents::LevelSet(LevelSet::V1(LevelSetV1 {
            level: sample_level(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "level-set");
        assert!(
            json["data"].get("level").is_none(),
            "flatten must elide the level envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "working");
        assert_eq!(json["data"]["description"], "Short-term");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn level_removed_round_trips_through_v1_layer() {
        let original = LevelEvents::LevelRemoved(LevelRemoved::V1(LevelRemovedV1 {
            name: LevelName::new("working"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: LevelEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            LevelEvents::LevelRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "working");
            }
            _ => panic!("wrong variant"),
        }
    }
}
