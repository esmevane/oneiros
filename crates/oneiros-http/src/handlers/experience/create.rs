use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<ExperienceResponses>), Error> {
    let response = ticket
        .service()
        .dispatch_experience(ExperienceRequests::CreateExperience(request))?;

    Ok((StatusCode::CREATED, Json(response)))
}
