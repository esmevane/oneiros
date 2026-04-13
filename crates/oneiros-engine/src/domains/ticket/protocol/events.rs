use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TicketEventsType, display = "kebab-case")]
pub(crate) enum TicketEvents {
    TicketIssued(Ticket),
    TicketUsed(TicketUsed),
    TicketRejected(TicketRejected),
}

/// Audit record: a ticket was successfully presented and validated.
/// Emitted on every successful auth that consumes a ticket's token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TicketUsed {
    pub(crate) ticket_id: TicketId,
    pub(crate) used_at: Timestamp,
}

/// Audit record: a ticket was presented but rejected. The reason is
/// human-readable and intended for logs and error surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TicketRejected {
    pub(crate) ticket_id: Option<TicketId>,
    pub(crate) reason: String,
    pub(crate) rejected_at: Timestamp,
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
