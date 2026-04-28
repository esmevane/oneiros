use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = UrgeEventsType, display = "kebab-case")]
pub enum UrgeEvents {
    UrgeSet(UrgeSet),
    UrgeRemoved(UrgeRemoved),
}

versioned! {
    pub enum UrgeSet {
        V1 => {
            #[serde(flatten)] pub urge: Urge,
        }
    }
}

versioned! {
    pub enum UrgeRemoved {
        V1 => {
            #[builder(into)] pub name: UrgeName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_urge() -> Urge {
        Urge::builder()
            .name("introspect")
            .description("The pull to look inward")
            .prompt("Pause and examine.")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (UrgeEventsType::UrgeSet, "urge-set"),
            (UrgeEventsType::UrgeRemoved, "urge-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn urge_set_wire_format_is_flat() {
        let event = UrgeEvents::UrgeSet(UrgeSet::V1(UrgeSetV1 {
            urge: sample_urge(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "urge-set");
        assert!(
            json["data"].get("urge").is_none(),
            "flatten must elide the urge envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "introspect");
        assert_eq!(json["data"]["description"], "The pull to look inward");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn urge_removed_round_trips_through_v1_layer() {
        let original = UrgeEvents::UrgeRemoved(UrgeRemoved::V1(UrgeRemovedV1 {
            name: UrgeName::new("introspect"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: UrgeEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            UrgeEvents::UrgeRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "introspect");
            }
            _ => panic!("wrong variant"),
        }
    }
}
