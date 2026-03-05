use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<UpdateExperienceSensationRequest>,
) -> Result<(StatusCode, Json<ExperienceResponses>), Error> {
    let experience = ticket.service().update_experience_sensation(&id, request)?;

    Ok((StatusCode::OK, Json(experience)))
}
