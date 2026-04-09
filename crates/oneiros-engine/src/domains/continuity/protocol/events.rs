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
pub struct ContinuityEvent {
    pub agent: AgentName,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensedEvent {
    pub agent: AgentName,
    pub content: Content,
    pub created_at: Timestamp,
}
