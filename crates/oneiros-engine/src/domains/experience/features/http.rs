use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub struct ExperienceRouter;

impl ExperienceRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
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
    agent: String,
    sensation: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    agent: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateDescriptionBody {
    description: String,
}

#[derive(Debug, Deserialize)]
struct UpdateSensationBody {
    sensation: String,
}

async fn create(
    State(context): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
    let response = ExperienceService::create(
        &context,
        &AgentName::new(&body.agent),
        SensationName::new(&body.sensation),
        Description::new(&body.description),
    )?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(context): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::list(
        &context,
        params.agent.as_deref().map(AgentName::new).as_ref(),
    )?))
}

async fn show(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    let id: ExperienceId = id
        .parse()
        .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?;
    Ok(Json(ExperienceService::get(&context, &id)?))
}

async fn update_description(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
    Json(body): Json<UpdateDescriptionBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    let id: ExperienceId = id
        .parse()
        .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?;
    Ok(Json(ExperienceService::update_description(
        &context,
        &id,
        Description::new(&body.description),
    )?))
}

async fn update_sensation(
    State(context): State<ProjectContext>,
    Path(id): Path<String>,
    Json(body): Json<UpdateSensationBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    let id: ExperienceId = id
        .parse()
        .map_err(|e: IdParseError| ExperienceError::Database(e.into()))?;
    Ok(Json(ExperienceService::update_sensation(
        &context,
        &id,
        SensationName::new(&body.sensation),
    )?))
}
