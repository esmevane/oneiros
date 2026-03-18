use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::contexts::ProjectContext;

use super::super::errors::CognitionError;
use super::super::responses::CognitionResponse;
use super::super::service::CognitionService;

pub fn routes() -> Router<ProjectContext> {
    Router::new()
        .route("/", routing::get(list).post(add))
        .route("/{id}", routing::get(show))
}

#[derive(Debug, Deserialize)]
struct AddBody {
    agent: String,
    texture: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<String>,
    texture: Option<String>,
}

async fn add(
    State(ctx): State<ProjectContext>,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
    let response = CognitionService::add(&ctx, body.agent, body.texture, body.content)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(ctx): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::list(
        &ctx,
        params.agent.as_deref(),
        params.texture.as_deref(),
    )?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::get(&ctx, &id)?))
}
