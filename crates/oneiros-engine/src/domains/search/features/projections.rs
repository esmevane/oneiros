use crate::*;

pub struct SearchProjections;

impl SearchProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

/// The search projection owns the FTS5 substrate (table, reset). Indexing
/// is driven by content-bearing domains, which call
/// [`SearchStore::index_expression`] from their own event handlers — so
/// `apply` here is a deliberate no-op.
const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    migrate: |conn| SearchStore::new(conn).migrate(),
    apply: |_conn, _event| Ok(()),
    reset: |conn| SearchStore::new(conn).reset(),
}];
