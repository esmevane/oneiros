use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<Response>), Error> {
    let response = ticket.dispatch(ExperienceRequests::CreateExperience(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
