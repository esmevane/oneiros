use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct MemoryRouter;

impl MemoryRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
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
    agent: String,
    level: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<String>,
}

async fn add(
    State(context): State<ProjectContext>,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<MemoryResponse>), MemoryError> {
    let response = MemoryService::add(
        &context,
        &AgentName::new(&body.agent),
        LevelName::new(&body.level),
        Content::new(body.content),
    )?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(context): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    Ok(Json(MemoryService::list(
        &context,
        params.agent.as_deref().map(AgentName::new).as_ref(),
    )?))
}

async fn show(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<MemoryResponse>, MemoryError> {
    let id: MemoryId = id
        .parse()
        .map_err(|e: IdParseError| MemoryError::Database(e.into()))?;
    Ok(Json(MemoryService::get(&context, &id)?))
}
