use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_texture(TextureRequests::GetTexture(GetTextureRequest { name }))?;

    Ok(Json(response))
}
