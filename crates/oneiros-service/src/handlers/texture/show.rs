use axum::{Json, extract::Path};
use oneiros_model::{TextureName, TextureRecord};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<TextureName>,
) -> Result<Json<TextureRecord>, Error> {
    let texture = ticket
        .db
        .get_texture(&given_name)?
        .ok_or(NotFound::Texture(given_name))?;

    Ok(Json(texture))
}
