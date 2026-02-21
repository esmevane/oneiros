use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddMemoryRequest>,
) -> Result<(StatusCode, Json<Identity<MemoryId, Memory>>), Error> {
    // Resolve agent name to agent_id.
    let agent = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(Key::Id(request.agent.clone())))?;

    // Validate that the referenced level exists.
    ticket
        .db
        .get_level(&request.level)?
        .ok_or(NotFound::Level(Key::Id(request.level.clone())))?;

    let memory = Identity::new(
        MemoryId::new(),
        Memory {
            agent_id: agent.id,
            level: request.level,
            content: request.content,
            created_at: Utc::now(),
        },
    );

    let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(memory)))
}
