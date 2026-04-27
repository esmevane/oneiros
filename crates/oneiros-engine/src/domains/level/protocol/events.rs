use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = LevelEventsType, display = "kebab-case")]
pub enum LevelEvents {
    LevelSet(Level),
    LevelRemoved(LevelRemoved),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LevelRemoved {
    Current(LevelRemovedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct LevelRemovedV1 {
    pub name: LevelName,
}

impl LevelRemoved {
    pub fn build_v1() -> LevelRemovedV1Builder {
        LevelRemovedV1::builder()
    }

    pub fn name(&self) -> &LevelName {
        match self {
            Self::Current(v) => &v.name,
        }
    }
}
