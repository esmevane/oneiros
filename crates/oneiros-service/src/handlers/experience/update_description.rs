use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::{Experience, ExperienceId};
use oneiros_protocol::{Events, ExperienceEvents, UpdateExperienceDescriptionRequest};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Validate that the experience exists.
    ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated {
        experience_id: id,
        description: request.description.clone(),
    });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    // Re-fetch the full experience (now includes the updated description via projection).
    let experience = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    Ok((StatusCode::OK, Json(experience)))
}
