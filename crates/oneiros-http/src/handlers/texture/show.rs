use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<TextureName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::GetTexture(
        GetTextureRequest { name },
    ))?))
}
