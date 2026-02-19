use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<AddExperienceRefRequest>,
) -> Result<(StatusCode, Json<Identity<ExperienceId, Experience>>), Error> {
    // Validate that the experience exists.
    ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let event = Events::Experience(ExperienceEvents::ExperienceRefAdded {
        experience_id: id,
        record_ref: request.clone(),
    });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    // Re-fetch the full experience (now includes the new ref via projection).
    let experience = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    Ok((StatusCode::OK, Json(experience)))
}
