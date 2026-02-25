use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

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

    let agent = Agent::init(
        request.description,
        request.prompt,
        request.name,
        request.persona,
    );

    let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
