use crate::store::Projection;

use super::super::repo::TextureRepo;

pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "texture",
        apply: |conn, event| TextureRepo::new(conn).handle(event),
        reset: |conn| TextureRepo::new(conn).reset(),
    },
];
