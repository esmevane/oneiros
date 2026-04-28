use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = NatureEventsType, display = "kebab-case")]
pub enum NatureEvents {
    NatureSet(NatureSet),
    NatureRemoved(NatureRemoved),
}

versioned! {
    pub enum NatureSet {
        V1 => {
            #[serde(flatten)] pub nature: Nature,
        }
    }
}

versioned! {
    pub enum NatureRemoved {
        V1 => {
            #[builder(into)] pub name: NatureName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nature() -> Nature {
        Nature::builder()
            .name("context")
            .description("Provides background")
            .prompt("Use when one thing frames another.")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (NatureEventsType::NatureSet, "nature-set"),
            (NatureEventsType::NatureRemoved, "nature-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn nature_set_wire_format_is_flat() {
        let event = NatureEvents::NatureSet(NatureSet::V1(NatureSetV1 {
            nature: sample_nature(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "nature-set");
        assert!(
            json["data"].get("nature").is_none(),
            "flatten must elide the nature envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "context");
        assert_eq!(json["data"]["description"], "Provides background");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn nature_removed_round_trips_through_v1_layer() {
        let original = NatureEvents::NatureRemoved(NatureRemoved::V1(NatureRemovedV1 {
            name: NatureName::new("context"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: NatureEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            NatureEvents::NatureRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "context");
            }
            _ => panic!("wrong variant"),
        }
    }
}
