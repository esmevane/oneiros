use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use serde::Deserialize;

use crate::*;

pub(crate) struct ExperienceRouter;

impl ExperienceRouter {
    pub(crate) fn routes(&self) -> Router<ServerState> {
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
struct UpdateDescriptionBody {
    description: Description,
}

#[derive(Debug, Deserialize)]
struct UpdateSensationBody {
    sensation: SensationName,
}

async fn create(
    context: ProjectContext,
    Json(body): Json<CreateExperience>,
) -> Result<(StatusCode, Json<ExperienceResponse>), ExperienceError> {
    let response = ExperienceService::create(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    context: ProjectContext,
    Query(params): Query<ListExperiences>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(ExperienceService::list(&context, &params).await?))
}

async fn show(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::get(&context, &GetExperience::builder().id(id).build()).await?,
    ))
}

async fn update_description(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateDescriptionBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_description(
            &context,
            &UpdateExperienceDescription::builder()
                .id(id)
                .description(body.description)
                .build(),
        )
        .await?,
    ))
}

async fn update_sensation(
    context: ProjectContext,
    Path(id): Path<ExperienceId>,
    Json(body): Json<UpdateSensationBody>,
) -> Result<Json<ExperienceResponse>, ExperienceError> {
    Ok(Json(
        ExperienceService::update_sensation(
            &context,
            &UpdateExperienceSensation::builder()
                .id(id)
                .sensation(body.sensation)
                .build(),
        )
        .await?,
    ))
}
