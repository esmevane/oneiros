use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PersonaEventsType, display = "kebab-case")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved(PersonaRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PersonaRemoved {
    Current(PersonaRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct PersonaRemovedV1 {
    pub name: PersonaName,
}

impl PersonaRemoved {
    pub fn build_v1() -> PersonaRemovedV1Builder {
        PersonaRemovedV1::builder()
    }

    pub fn name(&self) -> &PersonaName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}
