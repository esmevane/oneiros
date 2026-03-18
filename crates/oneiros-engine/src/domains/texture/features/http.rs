use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::TextureError;
use super::super::model::Texture;
use super::super::responses::TextureResponse;
use super::super::service::TextureService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{name}", routing::put(set).get(show).delete(remove))
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut texture): Json<Texture>,
) -> Result<(StatusCode, Json<TextureResponse>), TextureError> {
    texture.name = name;
    Ok((StatusCode::OK, Json(TextureService::set(&ctx, texture)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::get(&ctx, &name)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<TextureResponse>, TextureError> {
    Ok(Json(TextureService::remove(&ctx, &name)?))
}
