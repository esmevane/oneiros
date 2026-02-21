use axum::Json;
use oneiros_model::TextureRecord;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<TextureRecord>>, Error> {
    let textures = ticket.db.list_textures()?;

    Ok(Json(textures))
}
