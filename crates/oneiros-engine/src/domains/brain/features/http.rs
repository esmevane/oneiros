use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::SystemContext;

use super::super::errors::BrainError;
use super::super::responses::BrainResponse;
use super::super::service::BrainService;

pub fn routes() -> Router<SystemContext> {
    Router::new()
        .route("/", routing::get(list).post(create))
        .route("/{name}", routing::get(show))
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    name: String,
}

async fn create(
    State(ctx): State<SystemContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<BrainResponse>), BrainError> {
    let response = BrainService::create(&ctx, body.name)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<SystemContext>) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::list(&ctx)?))
}

async fn show(
    State(ctx): State<SystemContext>,
    Path(name): Path<String>,
) -> Result<Json<BrainResponse>, BrainError> {
    Ok(Json(BrainService::get(&ctx, &name)?))
}
