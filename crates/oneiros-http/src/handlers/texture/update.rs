use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<Json<TextureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_texture(TextureRequests::SetTexture(texture))?;

    Ok(Json(response))
}
