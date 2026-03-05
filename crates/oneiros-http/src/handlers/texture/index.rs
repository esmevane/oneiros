use axum::Json;
use oneiros_model::TextureResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<TextureResponses>, Error> {
    let textures = ticket.service().list_textures()?;

    Ok(Json(textures))
}
