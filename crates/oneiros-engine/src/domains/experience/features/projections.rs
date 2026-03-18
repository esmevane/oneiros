use crate::store::Projection;

use super::super::repo::ExperienceRepo;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "experience",
    apply: |conn, event| ExperienceRepo::new(conn).handle(event),
    reset: |conn| ExperienceRepo::new(conn).reset(),
}];
