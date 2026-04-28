use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PersonaEventsType, display = "kebab-case")]
pub enum PersonaEvents {
    PersonaSet(PersonaSet),
    PersonaRemoved(PersonaRemoved),
}

versioned! {
    pub enum PersonaSet {
        V1 => {
            #[serde(flatten)] pub persona: Persona,
        }
    }
}

versioned! {
    pub enum PersonaRemoved {
        V1 => {
            #[builder(into)] pub name: PersonaName,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_persona() -> Persona {
        Persona::builder()
            .name("process")
            .description("Process agents")
            .prompt("Manages internal lifecycle.")
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (PersonaEventsType::PersonaSet, "persona-set"),
            (PersonaEventsType::PersonaRemoved, "persona-removed"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }

    #[test]
    fn persona_set_wire_format_is_flat() {
        let event = PersonaEvents::PersonaSet(PersonaSet::V1(PersonaSetV1 {
            persona: sample_persona(),
        }));

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "persona-set");
        assert!(
            json["data"].get("persona").is_none(),
            "flatten must elide the persona envelope on the wire"
        );
        assert_eq!(json["data"]["name"], "process");
        assert_eq!(json["data"]["description"], "Process agents");
        assert!(
            json["data"].get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }

    #[test]
    fn persona_removed_round_trips_through_v1_layer() {
        let original = PersonaEvents::PersonaRemoved(PersonaRemoved::V1(PersonaRemovedV1 {
            name: PersonaName::new("process"),
        }));

        let json = serde_json::to_string(&original).unwrap();
        let decoded: PersonaEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            PersonaEvents::PersonaRemoved(removed) => {
                let v1 = removed.current().unwrap();
                assert_eq!(v1.name.to_string(), "process");
            }
            _ => panic!("wrong variant"),
        }
    }
}
