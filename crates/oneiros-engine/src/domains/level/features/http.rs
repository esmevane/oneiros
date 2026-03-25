use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct LevelRouter;

impl LevelRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/levels",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut level): Json<Level>,
) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
    level.name = LevelName::new(name);
    Ok((StatusCode::OK, Json(LevelService::set(&ctx, level).await?)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::get(&ctx, &LevelName::new(name))?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(
        LevelService::remove(&ctx, &LevelName::new(name)).await?,
    ))
}
