use axum::{Json, Router, extract::Path, http::StatusCode, routing};
use serde::Deserialize;

use crate::*;

pub struct AgentRouter;

impl AgentRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/agents",
            Router::<ServerState>::new()
                .route("/", routing::get(list).post(create))
                .route("/{name}", routing::get(show).put(update).delete(remove)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    name: AgentName,
    persona: PersonaName,
    description: Description,
    prompt: Prompt,
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
    let response = AgentService::create(
        &context,
        body.name,
        body.persona,
        body.description,
        body.prompt,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: ProjectContext) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::get(&context, &AgentName::new(&name)).await?,
    ))
}

#[derive(Debug, Deserialize)]
struct UpdateBody {
    persona: PersonaName,
    description: Description,
    prompt: Prompt,
}

async fn update(
    context: ProjectContext,
    Path(name): Path<String>,
    Json(body): Json<UpdateBody>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::update(
            &context,
            AgentName::new(&name),
            body.persona,
            body.description,
            body.prompt,
        )
        .await?,
    ))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<String>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::remove(&context, &AgentName::new(&name)).await?,
    ))
}
