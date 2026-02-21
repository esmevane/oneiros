use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<TextureName, TextureLink>>,
) -> Result<Json<Record<Texture>>, Error> {
    let texture = ticket
        .db
        .get_texture_by_key(&key)?
        .ok_or(NotFound::Texture(key))?;

    let record = Record::new(texture)?;
    Ok(Json(record))
}
