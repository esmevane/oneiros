use crate::store::Projection;

use super::super::repo::TenantRepo;

pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "tenant",
        apply: |conn, event| TenantRepo::new(conn).handle(event),
        reset: |conn| TenantRepo::new(conn).reset(),
    },
];
