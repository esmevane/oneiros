use crate::*;

pub(crate) struct AgentProjections;

impl AgentProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "agent",
    migrate: |conn| AgentStore::new(conn).migrate(),
    apply: |conn, event| AgentStore::new(conn).handle(event),
    reset: |conn| AgentStore::new(conn).reset(),
}];
