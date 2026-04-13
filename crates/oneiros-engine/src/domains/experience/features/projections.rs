use crate::*;

pub(crate) struct ExperienceProjections;

impl ExperienceProjections {
    pub(crate) const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "experience",
    migrate: |conn| ExperienceStore::new(conn).migrate(),
    apply: |conn, event| ExperienceStore::new(conn).handle(event),
    reset: |conn| ExperienceStore::new(conn).reset(),
}];
