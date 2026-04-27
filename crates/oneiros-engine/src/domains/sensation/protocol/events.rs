use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = SensationEventsType, display = "kebab-case")]
pub enum SensationEvents {
    SensationSet(Sensation),
    SensationRemoved(SensationRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensationRemoved {
    Current(SensationRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct SensationRemovedV1 {
    pub name: SensationName,
}

impl SensationRemoved {
    pub fn build_v1() -> SensationRemovedV1Builder {
        SensationRemovedV1::builder()
    }

    pub fn name(&self) -> &SensationName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}
