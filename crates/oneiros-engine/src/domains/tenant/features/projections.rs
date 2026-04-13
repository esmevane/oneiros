use crate::*;

pub(crate) struct TenantProjections;

impl TenantProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "tenant",
    migrate: |conn| TenantStore::new(conn).migrate(),
    apply: |conn, event| TenantStore::new(conn).handle(event),
    reset: |conn| TenantStore::new(conn).reset(),
}];
