use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::*;

pub struct SensationRouter;

impl SensationRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/sensations",
            Router::new()
                .route("/", routing::get(list))
                .route("/{name}", routing::put(set).get(show).delete(remove)),
        )
    }
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut sensation): Json<Sensation>,
) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
    sensation.name = SensationName::new(name);
    Ok((
        StatusCode::OK,
        Json(SensationService::set(&ctx, sensation).await?),
    ))
}

async fn list(
    State(ctx): State<ProjectContext>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::get(
        &ctx,
        &SensationName::new(name),
    )?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(
        SensationService::remove(&ctx, &SensationName::new(name)).await?,
    ))
}
