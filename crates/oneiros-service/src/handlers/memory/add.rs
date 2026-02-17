use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};
use oneiros_model::{AgentName, Content, Id, LevelName, Memory, MemoryId};
use oneiros_protocol::{AddMemoryRequest, Events, MemoryEvents};

use crate::*;

/// The identity-defining fields for a memory. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct MemoryContent<'a> {
    agent: &'a AgentName,
    level: &'a LevelName,
    content: &'a Content,
    created_at: &'a DateTime<Utc>,
}

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

    let created_at = Utc::now();
    let content_bytes = postcard::to_allocvec(&MemoryContent {
        agent: &request.agent,
        level: &request.level,
        content: &request.content,
        created_at: &created_at,
    })
    .expect("postcard serialization of memory content");

    let memory = Memory {
        id: MemoryId(Id::from_content(&content_bytes)),
        agent_id: agent.id,
        level: request.level,
        content: request.content,
        created_at,
    };

    let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(memory)))
}
