use axum::{Json, Router, extract::Path, routing};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::put(update))
        .route("/", routing::get(index))
        .route("/{name}", routing::get(show))
        .route("/{name}", routing::delete(delete))
}

async fn update(
    ticket: OneirosContext,
    Json(texture): Json<Texture>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::SetTexture(texture))?))
}

async fn index(ticket: OneirosContext) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::ListTextures(
        ListTexturesRequest,
    ))?))
}

async fn show(
    ticket: OneirosContext,
    Path(name): Path<TextureName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::GetTexture(
        GetTextureRequest { name },
    ))?))
}

async fn delete(
    ticket: OneirosContext,
    Path(name): Path<TextureName>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(TextureRequests::RemoveTexture(
        RemoveTextureRequest { name },
    ))?))
}
