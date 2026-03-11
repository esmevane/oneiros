use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing,
};
use oneiros_model::*;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new()
        .route("/", routing::post(create))
        .route("/", routing::get(index))
        .route("/{id}", routing::get(show))
        .route("/{id}/description", routing::put(update_description))
        .route("/{id}/sensation", routing::put(update_sensation))
}

async fn create(
    ticket: OneirosContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    let response = ticket.dispatch(ExperienceRequests::CreateExperience(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}

async fn index(
    ticket: OneirosContext,
    Query(request): Query<ListExperiencesRequest>,
) -> Result<Json<Response>, Error> {
    Ok(Json(
        ticket.dispatch(ExperienceRequests::ListExperiences(request))?,
    ))
}

async fn show(
    ticket: OneirosContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(ExperienceRequests::GetExperience(
        GetExperienceRequest { id },
    ))?))
}

async fn update_description(
    ticket: OneirosContext,
    Path(id): Path<ExperienceId>,
    Json(mut request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    request.id = id;

    Ok((
        StatusCode::OK,
        Json(ticket.dispatch(ExperienceRequests::UpdateExperienceDescription(request))?),
    ))
}

async fn update_sensation(
    ticket: OneirosContext,
    Path(id): Path<ExperienceId>,
    Json(mut request): Json<UpdateExperienceSensationRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    request.id = id;

    Ok((
        StatusCode::OK,
        Json(ticket.dispatch(ExperienceRequests::UpdateExperienceSensation(request))?),
    ))
}
