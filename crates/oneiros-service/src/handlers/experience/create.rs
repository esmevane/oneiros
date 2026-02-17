use axum::{Json, http::StatusCode};
use chrono::Utc;
use oneiros_model::{AgentName, Content, Experience, ExperienceId, Id, Link, SensationName};
use oneiros_protocol::{CreateExperienceRequest, Events, ExperienceEvents};

use crate::*;

/// The identity-defining fields for an experience. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct ExperienceContent<'a> {
    agent: &'a AgentName,
    sensation: &'a SensationName,
    description: &'a Content,
    links: &'a [Link],
}

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

    let content_bytes = postcard::to_allocvec(&ExperienceContent {
        agent: &request.agent,
        sensation: &request.sensation,
        description: &request.description,
        links: &request.links,
    })
    .expect("postcard serialization of experience content");

    let experience = Experience {
        id: ExperienceId(Id::from_content(&content_bytes)),
        agent_id: agent.id,
        sensation: request.sensation,
        description: request.description,
        links: request.links,
        created_at: Utc::now(),
    };

    let event = Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(experience)))
}
