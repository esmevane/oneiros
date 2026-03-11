use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::SetTexture(texture))?))
}
