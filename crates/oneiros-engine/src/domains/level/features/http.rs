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
    Json(mut body): Json<SetLevel>,
) -> Result<(StatusCode, Json<LevelResponse>), LevelError> {
    body.name = name;
    Ok((
        StatusCode::OK,
        Json(LevelService::set(&context, &body).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(LevelService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(
        LevelService::get(&context, &GetLevel::builder().name(name).build()).await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<LevelName>,
) -> Result<Json<LevelResponse>, LevelError> {
    Ok(Json(
        LevelService::remove(&context, &RemoveLevel::builder().name(name).build()).await?,
    ))
}
