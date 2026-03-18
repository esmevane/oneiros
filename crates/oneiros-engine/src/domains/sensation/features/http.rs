use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::SensationError;
use super::super::model::Sensation;
use super::super::responses::SensationResponse;
use super::super::service::SensationService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{name}", routing::put(set).get(show).delete(remove))
}

async fn set(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(mut sensation): Json<Sensation>,
) -> Result<(StatusCode, Json<SensationResponse>), SensationError> {
    sensation.name = name;
    Ok((
        StatusCode::OK,
        Json(SensationService::set(&ctx, sensation)?),
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
    Ok(Json(SensationService::get(&ctx, &name)?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<SensationResponse>, SensationError> {
    Ok(Json(SensationService::remove(&ctx, &name)?))
}
