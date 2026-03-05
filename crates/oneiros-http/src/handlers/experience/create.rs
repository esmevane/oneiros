use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<ExperienceResponses>), Error> {
    let experience = ticket.service().create_experience(request)?;

    Ok((StatusCode::CREATED, Json(experience)))
}
