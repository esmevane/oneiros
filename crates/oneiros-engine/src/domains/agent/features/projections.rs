use crate::store::Projection;

use super::super::repo::AgentRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "agent",
    apply: |conn, event| AgentRepo::new(conn).handle(event),
    reset: |conn| AgentRepo::new(conn).reset(),
}];
