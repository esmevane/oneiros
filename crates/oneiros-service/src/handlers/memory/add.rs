use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::{AgentId, Events, Memory, MemoryEvents, MemoryId};
use oneiros_protocol::AddMemoryRequest;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddMemoryRequest>,
) -> Result<(StatusCode, Json<Memory>), Error> {
    // Resolve agent name to agent_id.
    let (id, _, _, _, _) = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    let agent_id: AgentId = id.parse().unwrap_or_default();

    // Validate that the referenced level exists.
    ticket
        .db
        .get_level(&request.level)?
        .ok_or(NotFound::Level(request.level.clone()))?;

    let memory = Memory {
        id: MemoryId::new(),
        agent_id,
        level: request.level,
        content: request.content,
        created_at: Utc::now(),
    };

    let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(memory)))
}
