use axum::{Json, extract::Path};
use oneiros_model::{TextureName, TextureResponses};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<TextureName>,
) -> Result<Json<TextureResponses>, Error> {
    let texture = ticket.service().get_texture(&given_name)?;

    Ok(Json(texture))
}
