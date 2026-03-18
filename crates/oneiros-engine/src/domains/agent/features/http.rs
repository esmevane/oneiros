use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct AgentRouter;

impl AgentRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new().nest(
            "/agents",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{name}", routing::get(show).put(update).delete(remove)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    name: String,
    persona: String,
    description: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBody {
    persona: String,
    description: String,
    prompt: String,
}

async fn create(
    State(ctx): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
    let response =
        AgentService::create(&ctx, body.name, body.persona, body.description, body.prompt)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(ctx): State<ProjectContext>) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::list(&ctx)?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::get(&ctx, &name)?))
}

async fn update(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
    Json(body): Json<UpdateBody>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::update(
        &ctx,
        name,
        body.persona,
        body.description,
        body.prompt,
    )?))
}

async fn remove(
    State(ctx): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::remove(&ctx, &name)?))
}
