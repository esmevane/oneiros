use axum::{Json, http::StatusCode};
use oneiros_model::{Agent, AgentId, AgentName, Id, PersonaName};
use oneiros_protocol::{AgentEvents, CreateAgentRequest, Events};

use crate::*;

/// The identity-defining fields for an agent. Serialized with postcard
/// and hashed with SHA-256 to produce a deterministic content-addressed ID.
#[derive(serde::Serialize)]
struct AgentContent<'a> {
    name: &'a AgentName,
    persona: &'a PersonaName,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Agent>), Error> {
    // Validate that the referenced persona exists.
    ticket
        .db
        .get_persona(&request.persona)?
        .ok_or(NotFound::Persona(request.persona.clone()))?;

    // Check for name uniqueness.
    if ticket.db.agent_name_exists(&request.name)? {
        return Err(Conflicts::Agent(request.name).into());
    }

    let content_bytes = postcard::to_allocvec(&AgentContent {
        name: &request.name,
        persona: &request.persona,
    })
    .expect("postcard serialization of agent content");

    let agent = Agent {
        id: AgentId(Id::from_content(&content_bytes)),
        name: request.name,
        persona: request.persona,
        description: request.description,
        prompt: request.prompt,
    };

    let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
