use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct CognitionRouter;

impl CognitionRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/cognitions",
            Router::new()
                .route("/", routing::get(list).post(add))
                .route("/{id}", routing::get(show)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct AddBody {
    agent: AgentName,
    texture: TextureName,
    content: Content,
}

async fn add(
    context: ProjectContext,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
    let response = CognitionService::add(&context, body.agent, body.texture, body.content).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<AgentName>,
    texture: Option<TextureName>,
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListQuery>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::list(
        &context,
        params.agent,
        params.texture,
    )?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::get(&context, &id)?))
}
