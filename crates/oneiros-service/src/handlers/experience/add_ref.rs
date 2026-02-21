use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<AddExperienceRefRequest>,
) -> Result<(StatusCode, Json<Identity<ExperienceId, Experience>>), Error> {
    let id_str = id.to_string();

    // Validate that the experience exists.
    ticket
        .db
        .get_experience(&id_str)?
        .ok_or(NotFound::Experience(Key::Id(id.clone())))?;

    let event = Events::Experience(ExperienceEvents::ExperienceRefAdded {
        experience_id: id.clone(),
        record_ref: request.clone(),
        created_at: Some(chrono::Utc::now()),
    });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    // Re-fetch the full experience (now includes the new ref via projection).
    let experience = ticket
        .db
        .get_experience(&id_str)?
        .ok_or(NotFound::Experience(Key::Id(id)))?;

    Ok((StatusCode::OK, Json(experience)))
}
