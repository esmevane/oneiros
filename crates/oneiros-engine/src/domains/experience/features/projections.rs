use crate::*;

pub struct ExperienceProjections;

impl ExperienceProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "experience",
    migrate: |conn| ExperienceRepo::new(conn).migrate(),
    apply: |conn, event| ExperienceRepo::new(conn).handle(event),
    reset: |conn| ExperienceRepo::new(conn).reset(),
}];
