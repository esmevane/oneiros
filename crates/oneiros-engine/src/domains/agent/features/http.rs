use axum::{Json, Router, extract::Path, http::StatusCode, routing};

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

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateAgent>,
) -> Result<(StatusCode, Json<AgentResponse>), AgentError> {
    let response = AgentService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(context: ProjectContext) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::list(&context).await?))
}

async fn show(
    context: ProjectContext,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::get(&context, &GetAgent::builder().name(name).build()).await?,
    ))
}

async fn update(
    context: ProjectContext,
    Path(_): Path<AgentName>,
    Json(body): Json<UpdateAgent>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(AgentService::update(&context, &body).await?))
}

async fn remove(
    context: ProjectContext,
    Path(name): Path<AgentName>,
) -> Result<Json<AgentResponse>, AgentError> {
    Ok(Json(
        AgentService::remove(&context, &RemoveAgent::builder().name(name).build()).await?,
    ))
}
