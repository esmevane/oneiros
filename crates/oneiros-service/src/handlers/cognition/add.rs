use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

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

    let cognition = Cognition::create(agent.id, request.texture, request.content);

    let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));

    ticket.db.log_event(&event, projections::BRAIN)?;

    Ok((StatusCode::CREATED, Json(cognition)))
}
