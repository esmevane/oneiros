use crate::*;

pub struct TicketProjections;

impl TicketProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "ticket",
    apply: |conn, event| TicketRepo::new(conn).handle(event),
    reset: |conn| TicketRepo::new(conn).reset(),
}];
