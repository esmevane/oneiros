use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::{Events, Experience, ExperienceEvents, ExperienceId, RecordRef};
use oneiros_protocol::AddExperienceRefRequest;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<AddExperienceRefRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Validate that the experience exists.
    ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let record_ref = RecordRef {
        id: request.record_id,
        kind: request.record_kind,
        role: request.role,
    };

    let event = Events::Experience(ExperienceEvents::ExperienceRefAdded {
        experience_id: id,
        record_ref: record_ref.clone(),
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
