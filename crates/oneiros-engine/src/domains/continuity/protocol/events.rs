use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ContinuityEventsType, display = "kebab-case")]
pub enum ContinuityEvents {
    Dreamed(ContinuityEvent),
    Introspected(ContinuityEvent),
    Reflected(ContinuityEvent),
    Sensed(SensedEvent),
    Slept(ContinuityEvent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        let cases = [
            (ContinuityEventsType::Dreamed, "dreamed"),
            (ContinuityEventsType::Introspected, "introspected"),
            (ContinuityEventsType::Reflected, "reflected"),
            (ContinuityEventsType::Sensed, "sensed"),
            (ContinuityEventsType::Slept, "slept"),
        ];
        for (event_type, expectation) in cases {
            assert_eq!(&event_type.to_string(), expectation);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContinuityEvent {
    Current(ContinuityEventV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct ContinuityEventV1 {
    pub agent: AgentName,
    pub created_at: Timestamp,
}

impl ContinuityEvent {
    pub fn build_v1() -> ContinuityEventV1Builder {
        ContinuityEventV1::builder()
    }

    pub fn agent(&self) -> &AgentName {
        match self {
            Self::Current(v) => &v.agent,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SensedEvent {
    Current(SensedEventV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct SensedEventV1 {
    pub agent: AgentName,
    pub content: Content,
    pub created_at: Timestamp,
}

impl SensedEvent {
    pub fn build_v1() -> SensedEventV1Builder {
        SensedEventV1::builder()
    }

    pub fn agent(&self) -> &AgentName {
        match self {
            Self::Current(v) => &v.agent,
        }
    }

    pub fn content(&self) -> &Content {
        match self {
            Self::Current(v) => &v.content,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
}
