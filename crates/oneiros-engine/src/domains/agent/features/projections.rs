use crate::*;

pub struct AgentProjections;

impl AgentProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "agent",
    migrate: |conn| AgentRepo::new(conn).migrate(),
    apply: |conn, event| AgentRepo::new(conn).handle(event),
    reset: |conn| AgentRepo::new(conn).reset(),
}];
