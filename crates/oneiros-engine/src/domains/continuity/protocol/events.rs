use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ContinuityEventsType, display = "kebab-case")]
pub(crate) enum ContinuityEvents {
    Dreamed(Dreamed),
    Introspected(Introspected),
    Reflected(Reflected),
    Sensed(Sensed),
    Slept(Slept),
}

versioned! {
    pub(crate) enum Dreamed {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum Introspected {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum Reflected {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum Sensed {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) content: Content,
            pub(crate) created_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum Slept {
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            pub(crate) created_at: Timestamp,
        }
    }
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
