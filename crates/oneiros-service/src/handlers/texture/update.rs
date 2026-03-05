use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<Json<TextureResponses>, Error> {
    let texture = ticket.service().set_texture(texture)?;

    Ok(Json(texture))
}
