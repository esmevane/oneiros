use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::{Agent, AgentName};
use oneiros_protocol::{AgentEvents, Events, UpdateAgentRequest};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<AgentName>,
    Json(request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<Agent>), Error> {
    // Validate that the agent exists and get its current data (especially the ID).
    let existing = ticket
        .db
        .get_agent(&given_name)?
        .ok_or(NotFound::Agent(given_name))?;

    // Validate that the referenced persona exists.
    ticket
        .db
        .get_persona(&request.persona)?
        .ok_or(NotFound::Persona(request.persona.clone()))?;

    let agent = Agent {
        id: existing.id,
        name: existing.name,
        persona: request.persona,
        description: request.description,
        prompt: request.prompt,
    };

    let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::OK, Json(agent)))
}
