use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = UrgeEventsType, display = "kebab-case")]
pub enum UrgeEvents {
    UrgeSet(Urge),
    UrgeRemoved(UrgeRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgeRemoved {
    pub name: UrgeName,
}
