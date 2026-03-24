use crate::*;

pub struct TenantProjections;

impl TenantProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "tenant",
    migrate: |conn| TenantRepo::new(conn).migrate(),
    apply: |conn, event| TenantRepo::new(conn).handle(event),
    reset: |conn| TenantRepo::new(conn).reset(),
}];
