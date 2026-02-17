use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_client::CreateExperienceRequest;
use oneiros_model::{AgentId, Events, Experience, ExperienceEvents, ExperienceId};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateExperienceRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Resolve agent name to agent_id.
    let (id, _, _, _, _) = ticket
        .db
        .get_agent(&request.agent)?
        .ok_or(NotFound::Agent(request.agent.clone()))?;

    let agent_id: AgentId = id.parse().unwrap_or_default();

    // Validate that the referenced sensation exists.
    ticket
        .db
        .get_sensation(&request.sensation)?
        .ok_or(NotFound::Sensation(request.sensation.clone()))?;

    let experience = Experience {
        id: ExperienceId::new(),
        agent_id,
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
