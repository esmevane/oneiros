use crate::*;
use axum::{Json, http::StatusCode};
use oneiros_model::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddMemoryRequest>,
) -> Result<(StatusCode, Json<Memory>), Error> {
    // Resolve agent name to agent_id.
    let agent = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    // Validate that the referenced level exists.
    ticket
        .db
        .get_level(&request.level)?
        .ok_or(NotFound::Level(request.level.clone()))?;

    let memory = Memory::create(agent.id, request.level, request.content);

    let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));

    ticket.db.log_event(&event, projections::BRAIN)?;
    ticket.broadcast(&event);

    Ok((StatusCode::CREATED, Json(memory)))
}
