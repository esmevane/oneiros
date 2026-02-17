use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};
use oneiros_model::{AgentName, Cognition, CognitionId, Content, Id, TextureName};
use oneiros_protocol::{AddCognitionRequest, CognitionEvents, Events};

use crate::*;

/// The identity-defining fields for a cognition. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct CognitionContent<'a> {
    agent: &'a AgentName,
    texture: &'a TextureName,
    content: &'a Content,
    created_at: &'a DateTime<Utc>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Cognition>), Error> {
    // Resolve agent name to agent_id.
    let agent = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    // Validate that the referenced texture exists.
    ticket
        .db
        .get_texture(&request.texture)?
        .ok_or(NotFound::Texture(request.texture.clone()))?;

    let created_at = Utc::now();
    let content_bytes = postcard::to_allocvec(&CognitionContent {
        agent: &request.agent,
        texture: &request.texture,
        content: &request.content,
        created_at: &created_at,
    })
    .expect("postcard serialization of cognition content");

    let cognition = Cognition {
        id: CognitionId(Id::from_content(&content_bytes)),
        agent_id: agent.id,
        texture: request.texture,
        content: request.content,
        created_at,
    };

    let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(cognition)))
}
