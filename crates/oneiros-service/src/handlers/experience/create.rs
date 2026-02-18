use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
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
        id: ExperienceId::new(),
        agent_id: agent.id,
        sensation: request.sensation,
        description: request.description,
        refs: request.refs,
        created_at: Utc::now(),
    };

    let event = Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(experience)))
}
