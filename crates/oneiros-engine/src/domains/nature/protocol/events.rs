use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = NatureEventsType, display = "kebab-case")]
pub enum NatureEvents {
    NatureSet(Nature),
    NatureRemoved(NatureRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatureRemoved {
    pub name: NatureName,
}
