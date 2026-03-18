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
    State(ctx): State<ProjectContext>,
    Json(body): Json<CreateBody>,
) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
    let response = ExperienceService::create(&ctx, body.agent, body.sensation, body.description)?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    State(ctx): State<ProjectContext>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::list(
        &ctx,
        params.agent.as_deref(),
    )?))
}

async fn show(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::get(&ctx, &id)?))
}

async fn update_description(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
    Json(body): Json<UpdateDescriptionBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::update_description(
        &ctx,
        &id,
        body.description,
    )?))
}

async fn update_sensation(
    State(ctx): State<ProjectContext>,
    Path(id): Path<String>,
    Json(body): Json<UpdateSensationBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::update_sensation(
        &ctx,
        &id,
        body.sensation,
    )?))
}
