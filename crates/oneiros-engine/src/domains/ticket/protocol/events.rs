use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(
    kind = TicketEventsType,
    display = "kebab-case",
    attrs(
        expect(
            clippy::enum_variant_names,
            reason = "We use these for `type` notation in serde"
        )
    )
)]
#[expect(
    clippy::enum_variant_names,
    reason = "We use these for `type` notation in serde"
)]
pub(crate) enum TicketEvents {
    TicketIssued(TicketIssued),
    TicketUsed(TicketUsed),
    TicketRejected(TicketRejected),
}

impl TicketEvents {
    pub(crate) fn maybe_ticket(&self) -> Option<Ticket> {
        match self {
            TicketEvents::TicketIssued(event) => event.clone().current().ok().map(|v| v.ticket),
            TicketEvents::TicketUsed(_) | TicketEvents::TicketRejected(_) => None,
        }
    }
}

versioned! {
    pub(crate) enum TicketIssued {
        V1 => {
            #[serde(flatten)] pub(crate) ticket: Ticket,
        }
    }
}

versioned! {
    pub(crate) enum TicketUsed {
        V1 => {
            pub(crate) ticket_id: TicketId,
            pub(crate) used_at: Timestamp,
        }
    }
}

versioned! {
    pub(crate) enum TicketRejected {
        V1 => {
            pub(crate) ticket_id: Option<TicketId>,
            pub(crate) reason: String,
            pub(crate) rejected_at: Timestamp,
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
            .project_name(ProjectName::new("test-project"))
            .project_id(ProjectId::new())
            .link(Link::new(
                Ref::project(ProjectId::new()),
                Token::from("token"),
            ))
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
    fn legacy_ticket_issued_with_brain_fields_decodes_via_v1() {
        // Construct the legacy wire by serializing a current ticket and renaming
        // `project_name`/`project_id` to `brain_name`/`brain_id`. Keeps the rest of
        // the payload (`link`, `granted_by`, etc.) in whatever shape the current
        // serde impl produces, so we're only testing the renamed-field path.
        let ticket = sample_ticket();
        let mut data = serde_json::to_value(&ticket).unwrap();
        let obj = data.as_object_mut().unwrap();
        let project_name = obj.remove("project_name").unwrap();
        let project_id = obj.remove("project_id").unwrap();
        obj.insert("brain_name".into(), project_name);
        obj.insert("brain_id".into(), project_id);

        let json = serde_json::json!({ "type": "ticket-issued", "data": data });
        let event: TicketEvents = serde_json::from_value(json).expect("legacy decode");
        let issued = match event {
            TicketEvents::TicketIssued(inner) => inner.current().unwrap(),
            other => panic!("expected TicketIssued, got {other:?}"),
        };
        assert_eq!(issued.ticket.project_name, ticket.project_name);
        assert_eq!(issued.ticket.project_id, ticket.project_id);
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
        assert_eq!(json["data"]["project_name"], "test-project");
        assert!(json["data"].get("created_at").is_some());
    }
}
