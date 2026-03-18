use axum::{
    Json, Router,
    extract::{Path, State},
    routing,
};

use crate::contexts::ProjectContext;

use super::super::errors::PressureError;
use super::super::responses::PressureResponse;
use super::super::service::PressureService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list))
        .route("/{agent}", routing::get(get))
}

async fn get(
    State(ctx): State<ProjectContext>,
    Path(agent): Path<String>,
) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::get(&ctx, &agent)?))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<PressureResponse>, PressureError> {
    Ok(Json(PressureService::list(&ctx)?))
}
