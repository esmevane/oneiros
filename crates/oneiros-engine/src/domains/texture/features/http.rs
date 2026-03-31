use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct TextureRouter;

impl TextureRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/textures",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<TextureName>,
    Json(mut body): Json<SetTexture>,
) -> Result<(StatusCode, Json<TextureResponse>), TextureError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(TextureService::set(&context, &body).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::get(&context, &GetTexture::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<TextureName>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(
        TextureService::remove(&context, &RemoveTexture::builder().name(name).build()).await?,
    ))
}
