use bon::Builder;
use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TicketEventsType, display = "kebab-case")]
pub enum TicketEvents {
    TicketIssued(Ticket),
    TicketUsed(TicketUsed),
    TicketRejected(TicketRejected),
}

/// Audit record: a ticket was successfully presented and validated.
/// Emitted on every successful auth that consumes a ticket's token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TicketUsed {
    Current(TicketUsedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct TicketUsedV1 {
    pub ticket_id: TicketId,
    pub used_at: Timestamp,
}

impl TicketUsed {
    pub fn build_v1() -> TicketUsedV1Builder {
        TicketUsedV1::builder()
    }

    pub fn ticket_id(&self) -> TicketId {
        match self {
            Self::Current(v) => v.ticket_id,
        }
    }

    pub fn used_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.used_at,
        }
    }
}

/// Audit record: a ticket was presented but rejected. The reason is
/// human-readable and intended for logs and error surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TicketRejected {
    Current(TicketRejectedV1),
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct TicketRejectedV1 {
    pub ticket_id: Option<TicketId>,
    pub reason: String,
    pub rejected_at: Timestamp,
}

impl TicketRejected {
    pub fn build_v1() -> TicketRejectedV1Builder {
        TicketRejectedV1::builder()
    }

    pub fn ticket_id(&self) -> Option<TicketId> {
        match self {
            Self::Current(v) => v.ticket_id,
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::Current(v) => &v.reason,
        }
    }

    pub fn rejected_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.rejected_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&TicketEventsType::TicketIssued.to_string(), "ticket-issued");
        assert_eq!(&TicketEventsType::TicketUsed.to_string(), "ticket-used");
        assert_eq!(
            &TicketEventsType::TicketRejected.to_string(),
            "ticket-rejected"
        );
    }
}
