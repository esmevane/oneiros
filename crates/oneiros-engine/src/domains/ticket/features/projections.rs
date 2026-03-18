use crate::store::Projection;

use super::super::repo::TicketRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "ticket",
    apply: |conn, event| TicketRepo::new(conn).handle(event),
    reset: |conn| TicketRepo::new(conn).reset(),
}];
