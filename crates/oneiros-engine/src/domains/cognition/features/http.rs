use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct CognitionRouter;

impl CognitionRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
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
    State(context): State<ProjectContext>,
    Json(body): Json<AddBody>,
) -> Result<(StatusCode, Json<CognitionResponse>), CognitionError> {
    let response = CognitionService::add(
        &context,
        &AgentName::new(&body.agent),
        TextureName::new(&body.texture),
        Content::new(body.content),
    )
    .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(context): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    Ok(Json(CognitionService::list(
        &context,
        params.agent.as_deref().map(AgentName::new).as_ref(),
        params.texture.as_deref().map(TextureName::new).as_ref(),
    )?))
}

async fn show(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<CognitionResponse>, CognitionError> {
    let id: CognitionId = id
        .parse()
        .map_err(|e: IdParseError| CognitionError::Database(e.into()))?;
    Ok(Json(CognitionService::get(&context, &id)?))
}
