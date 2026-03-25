use crate::*;

pub struct TextureProjections;

impl TextureProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "texture",
    migrate: |conn| TextureStore::new(conn).migrate(),
    apply: |conn, event| TextureStore::new(conn).handle(event),
    reset: |conn| TextureStore::new(conn).reset(),
}];
