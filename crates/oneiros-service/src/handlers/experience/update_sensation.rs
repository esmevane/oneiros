use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<UpdateExperienceSensationRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Validate that the experience exists.
    ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated {
        experience_id: id,
        sensation: request.sensation.clone(),
    });

    ticket.db.log_event(&event, projections::BRAIN)?;
    ticket.broadcast(&event);

    // Re-fetch the full experience (now includes the updated sensation via projection).
    let experience = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    Ok((StatusCode::OK, Json(experience)))
}
