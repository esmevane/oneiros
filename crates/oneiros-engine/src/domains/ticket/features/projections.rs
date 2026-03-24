use crate::*;

pub struct TicketProjections;

impl TicketProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "ticket",
    migrate: |conn| TicketRepo::new(conn).migrate(),
    apply: |conn, event| TicketRepo::new(conn).handle(event),
    reset: |conn| TicketRepo::new(conn).reset(),
}];
