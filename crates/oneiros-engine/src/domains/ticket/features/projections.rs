use crate::*;

pub(crate) struct TicketProjections;

impl TicketProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "ticket",
    migrate: |conn| TicketStore::new(conn).migrate(),
    apply: |conn, event| TicketStore::new(conn).handle(event),
    reset: |conn| TicketStore::new(conn).reset(),
}];
