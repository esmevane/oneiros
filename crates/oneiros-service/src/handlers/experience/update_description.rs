use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<ExperienceId, ExperienceLink>>,
    Json(request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Identity<ExperienceId, Experience>>), Error> {
    let experience = ticket
        .db
        .get_experience_by_key(&key)?
        .ok_or(NotFound::Experience(key.clone()))?;

    let id = experience.id.clone();

    let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated {
        experience_id: id.clone(),
        description: request.description.clone(),
    });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    // Re-fetch the full experience (now includes the updated description via projection).
    let experience = ticket
        .db
        .get_experience(&id)?
        .ok_or(NotFound::Experience(key))?;

    Ok((StatusCode::OK, Json(experience)))
}
