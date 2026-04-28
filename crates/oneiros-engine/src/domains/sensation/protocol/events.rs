use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = SensationEventsType, display = "kebab-case")]
pub enum SensationEvents {
    SensationSet(SensationSet),
    SensationRemoved(SensationRemoved),
}

versioned! {
    pub enum SensationSet {
        V1 => {
            #[serde(flatten)] pub sensation: Sensation,
        }
    }
}

versioned! {
    pub enum SensationRemoved {
        V1 => {
            #[builder(into)] pub name: SensationName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sensation() -> Sensation {
        Sensation::builder()
            .name("echoes")
            .description("Resonance between thoughts")
            .prompt("Use when things rhyme.")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (SensationEventsType::SensationSet, "sensation-set"),
            (SensationEventsType::SensationRemoved, "sensation-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn sensation_set_wire_format_is_flat() {
        let event = SensationEvents::SensationSet(SensationSet::V1(SensationSetV1 {
            sensation: sample_sensation(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "sensation-set");
        assert!(
            json["data"].get("sensation").is_none(),
            "flatten must elide the sensation envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "echoes");
        assert_eq!(json["data"]["description"], "Resonance between thoughts");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn sensation_removed_round_trips_through_v1_layer() {
        let original =
            SensationEvents::SensationRemoved(SensationRemoved::V1(SensationRemovedV1 {
                name: SensationName::new("echoes"),
            }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: SensationEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            SensationEvents::SensationRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "echoes");
            }
            _ => panic!("wrong variant"),
        }
    }
}
