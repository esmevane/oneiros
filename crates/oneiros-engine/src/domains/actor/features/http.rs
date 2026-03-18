use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::SystemContext;

use super::super::errors::ActorError;
use super::super::responses::ActorResponse;
use super::super::service::ActorService;

pub fn routes() -> Router<SystemContext> {
    Router::new()
        .route("/", routing::get(list).post(create))
        .route("/{id}", routing::get(show))
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    tenant_id: String,
    name: String,
}

async fn create(
    State(ctx): State<SystemContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ActorResponse>), ActorError> {
    let response = ActorService::create(&ctx, body.tenant_id, body.name)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(ctx): State<SystemContext>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::list(&ctx)?))
}

async fn show(
    State(ctx): State<SystemContext>,
    Path(id): Path<String>,
) -> Result<Json<ActorResponse>, ActorError> {
    Ok(Json(ActorService::get(&ctx, &id)?))
}
