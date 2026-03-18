use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::ProjectContext;

use super::super::errors::MemoryError;
use super::super::responses::MemoryResponse;
use super::super::service::MemoryService;

pub const PATH: &str = "/memories";

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list).post(add))
        .route("/{id}", routing::get(show))
}

#[derive(Debug, Deserialize)]
struct AddBody {
    agent: String,
    level: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<String>,
}

async fn add(
    State(ctx): State<ProjectContext>,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(&ctx, body.agent, body.level, body.content)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(ctx): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::list(&ctx, params.agent.as_deref())?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::get(&ctx, &id)?))
}
