use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<AddCognitionRequest>,
) -> Result<(StatusCode, Json<Record<CognitionId, Cognition>>), Error> {
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

    let cognition = Record::create(Cognition {
        agent_id: Key::Id(agent.id),
        texture: request.texture,
        content: request.content,
    });

    let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(cognition)))
}
