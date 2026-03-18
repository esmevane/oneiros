use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::LevelError;
use super::super::model::Level;
use super::super::responses::LevelResponse;
use super::super::service::LevelService;

pub const PATH: &str = "/levels";

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{name}", routing::put(set).get(show).delete(remove))
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut level): Json<Level>,
) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
    level.name = name;
    Ok((StatusCode::OK, Json(LevelService::set(&ctx, level)?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::get(&ctx, &name)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::remove(&ctx, &name)?))
}
