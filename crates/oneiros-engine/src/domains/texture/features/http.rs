use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct TextureRouter;

impl TextureRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/textures",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut texture): Json<Texture>,
) -> Result<(StatusCode, Json<TextureResponse>), TextureError> {
    texture.name = TextureName::new(name);
    Ok((StatusCode::OK, Json(TextureService::set(&ctx, texture)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::get(&ctx, &TextureName::new(name))?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::remove(&ctx, &TextureName::new(name))?))
}
