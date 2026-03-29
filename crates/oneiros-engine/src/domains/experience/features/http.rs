use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ExperienceRouter;

impl ExperienceRouter {
    pub fn routes(&self) -> Router<ServerState> {
        Router::new().nest(
            "/experiences",
            Router::new()
                .route("/", routing::get(list).post(create))
                .route("/{id}", routing::get(show))
                .route("/{id}/description", routing::put(update_description))
                .route("/{id}/sensation", routing::put(update_sensation)),
        )
    }
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    agent: AgentName,
    sensation: SensationName,
    description: Description,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<AgentName>,
}

#[derive(Debug, Deserialize)]
struct UpdateDescriptionBody {
    description: Description,
}

#[derive(Debug, Deserialize)]
struct UpdateSensationBody {
    sensation: SensationName,
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
    let response =
        ExperienceService::create(&context, body.agent, body.sensation, body.description).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListQuery>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::list(&context, params.agent)?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::get(&context, &id)?))
}

async fn update_description(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateDescriptionBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_description(&context, &id, body.description).await?,
    ))
}

async fn update_sensation(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateSensationBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_sensation(&context, &id, body.sensation).await?,
    ))
}
