use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct LevelRouter;

impl LevelRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/levels",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<LevelName>,
    Json(mut level): Json<Level>,
) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
    level.name = name;
    Ok((
        StatusCode::OK,
        Json(LevelService::set(&context, level).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::list(&context)?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::get(&context, &name)?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::remove(&context, &name).await?))
}
