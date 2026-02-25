use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<ExperienceRecord>), Error> {
    // Resolve agent name to agent_id.
    let agent = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    // Validate that the referenced sensation exists.
    ticket
        .db
        .get_sensation(&request.sensation)?
        .ok_or(NotFound::Sensation(request.sensation.clone()))?;

    let experience = Experience {
        agent_id: agent.id,
        sensation: request.sensation,
    };

    let record = ExperienceRecord::init(request.description, request.refs, experience);

    let event = Events::Experience(ExperienceEvents::ExperienceCreated(record.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(record)))
}
