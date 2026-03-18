use crate::*;

pub struct TenantProjections;

impl TenantProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "tenant",
    apply: |conn, event| TenantRepo::new(conn).handle(event),
    reset: |conn| TenantRepo::new(conn).reset(),
}];
