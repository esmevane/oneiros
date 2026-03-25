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
    State(context): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
    let response = AgentService::create(
        &context,
        AgentName::new(&body.name),
        PersonaName::new(&body.persona),
        Description::new(&body.description),
        Prompt::new(&body.prompt),
    )
    .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(State(context): State<ProjectContext>) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::list(&context)?))
}

async fn show(
    State(context): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::get(&context, &AgentName::new(&name))?))
}

async fn update(
    State(context): State<ProjectContext>,
    Path(name): Path<String>,
    Json(body): Json<UpdateBody>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::update(
            &context,
            AgentName::new(&name),
            PersonaName::new(&body.persona),
            Description::new(&body.description),
            Prompt::new(&body.prompt),
        )
        .await?,
    ))
}

async fn remove(
    State(context): State<ProjectContext>,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::remove(&context, &AgentName::new(&name)).await?,
    ))
}
