use axum::{Json, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

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

    let experience = ExperienceRecord::init(
        request.description,
        request.refs,
        Experience {
            agent_id: Key::Id(agent.id),
            sensation: request.sensation,
        },
    );

    let event = Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(experience)))
}
