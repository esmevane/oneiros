use axum::Json;
use oneiros_model::Texture;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Texture>>, Error> {
    let textures = ticket.service().list_textures()?;

    Ok(Json(textures))
}
