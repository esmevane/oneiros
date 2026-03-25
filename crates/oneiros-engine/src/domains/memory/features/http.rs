use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct MemoryRouter;

impl MemoryRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/memories",
            Router::new()
                .route("/", routing::get(list).post(add))
                .route("/{id}", routing::get(show)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct AddBody {
    agent: AgentName,
    level: LevelName,
    content: Content,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<AgentName>,
}

async fn add(
    context: ProjectContext,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(&context, body.agent, body.level, body.content).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListQuery>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::list(&context, params.agent).await?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<MemoryId>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::get(&context, &id).await?))
}
