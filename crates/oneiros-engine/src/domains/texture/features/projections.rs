use crate::*;

pub struct TextureProjections;

impl TextureProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "texture",
    apply: |conn, event| TextureRepo::new(conn).handle(event),
    reset: |conn| TextureRepo::new(conn).reset(),
}];
