//! Ticket view — presentation authority for the ticket domain.
//!
//! Maps ticket responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

use crate::*;

pub struct TicketView;

impl TicketView {
    /// Table of tickets with standard columns.
    pub fn table(tickets: &Listed<Ticket>) -> Table {
        let mut table = Table::new(vec![
            Column::key("brain_name", "Brain"),
            Column::key("actor_id", "Actor"),
        ]);

        for ticket in &tickets.items {
            table.push_row(vec![
                ticket.brain_name.to_string(),
                ticket.actor_id.to_string(),
            ]);
        }

        table
    }

    /// Detail view for a single ticket.
    pub fn detail(ticket: &Ticket) -> Detail {
        Detail::new(ticket.brain_name.to_string()).field("actor_id:", ticket.actor_id.to_string())
    }

    /// Confirmation for a mutation.
    pub fn confirmed(verb: &str, ticket: &Ticket) -> Confirmation {
        Confirmation::new("Ticket", ticket.brain_name.to_string(), verb)
    }
}
