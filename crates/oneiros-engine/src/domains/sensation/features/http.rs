use axum::{Json, Router, extract::Path, http::StatusCode, routing};

use crate::*;

pub struct SensationRouter;

impl SensationRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/sensations",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    context: ProjectContext,
    Path(name): Path<SensationName>,
    Json(mut sensation): Json<Sensation>,
) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
    sensation.name = name;
    Ok((
        StatusCode::OK,
        Json(SensationService::set(&context, sensation).await?),
    ))
}

async fn list(context: ProjectContext) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::get(&context, &name).await?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<SensationName>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::remove(&context, &name).await?))
}
