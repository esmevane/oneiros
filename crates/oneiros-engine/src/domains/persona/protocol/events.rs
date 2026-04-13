use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = PersonaEventsType, display = "kebab-case")]
pub(crate) enum PersonaEvents {
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
pub(crate) struct PersonaRemoved {
    pub(crate) name: PersonaName,
}
