use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TicketEventsType, display = "kebab-case")]
pub enum TicketEvents {
    TicketIssued(TicketIssued),
    TicketUsed(TicketUsed),
    TicketRejected(TicketRejected),
}

impl TicketEvents {
    pub fn maybe_ticket(&self) -> Option<Ticket> {
        match self {
            TicketEvents::TicketIssued(event) => event.clone().current().ok().map(|v| v.ticket),
            TicketEvents::TicketUsed(_) | TicketEvents::TicketRejected(_) => None,
        }
    }
}

versioned! {
    pub enum TicketIssued {
        V1 => {
            #[serde(flatten)] pub ticket: Ticket,
        }
    }
}

versioned! {
    pub enum TicketUsed {
        V1 => {
            pub ticket_id: TicketId,
            pub used_at: Timestamp,
        }
    }
}

versioned! {
    pub enum TicketRejected {
        V1 => {
            pub ticket_id: Option<TicketId>,
            pub reason: String,
            pub rejected_at: Timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ticket() -> Ticket {
        let actor_id = ActorId::new();
        Ticket::builder()
            .actor_id(actor_id)
            .brain_name(BrainName::new("test-brain"))
            .brain_id(BrainId::new())
            .link(Link::new(Ref::brain(BrainId::new()), Token::from("token")))
            .granted_by(actor_id)
            .build()
    }

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&TicketEventsType::TicketIssued.to_string(), "ticket-issued");
        assert_eq!(&TicketEventsType::TicketUsed.to_string(), "ticket-used");
        assert_eq!(
            &TicketEventsType::TicketRejected.to_string(),
            "ticket-rejected"
        );
    }

    #[test]
    fn ticket_issued_wire_format_is_flat() {
        let ticket = sample_ticket();
        let event = TicketEvents::TicketIssued(TicketIssued::V1(TicketIssuedV1 {
            ticket: ticket.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "ticket-issued");
        assert!(
            json["data"].get("ticket").is_none(),
            "flatten must elide the ticket envelope on the wire"
        );
        assert_eq!(json["data"]["id"], ticket.id.to_string());
        assert_eq!(json["data"]["brain_name"], "test-brain");
        assert!(json["data"].get("created_at").is_some());
    }
}
