use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::{AgentId, Cognition, CognitionEvents, CognitionId, Events};
use oneiros_protocol::AddCognitionRequest;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Cognition>), Error> {
    // Resolve agent name to agent_id.
    let (id, _, _, _, _) = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    let agent_id: AgentId = id.parse().unwrap_or_default();

    // Validate that the referenced texture exists.
    ticket
        .db
        .get_texture(&request.texture)?
        .ok_or(NotFound::Texture(request.texture.clone()))?;

    let cognition = Cognition {
        id: CognitionId::new(),
        agent_id,
        texture: request.texture,
        content: request.content,
        created_at: Utc::now(),
    };

    let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(cognition)))
}
