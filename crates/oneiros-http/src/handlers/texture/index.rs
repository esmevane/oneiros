use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<TextureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_texture(TextureRequests::ListTextures(ListTexturesRequest))?;

    Ok(Json(response))
}
