use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Identity<ExperienceId, Experience>>), Error> {
    let id_str = id.to_string();

    // Validate that the experience exists.
    ticket
        .db
        .get_experience(&id_str)?
        .ok_or(NotFound::Experience(id_str.clone()))?;

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
        .get_experience(&id_str)?
        .ok_or(NotFound::Experience(id_str))?;

    Ok((StatusCode::OK, Json(experience)))
}
