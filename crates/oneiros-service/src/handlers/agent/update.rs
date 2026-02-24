use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<AgentName>,
    Json(request): Json<UpdateAgentRequest>,
) -> Result<(StatusCode, Json<AgentRecord>), Error> {
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

    let agent = AgentRecord::construct(
        existing.id,
        request.description.as_str(),
        request.prompt.as_str(),
        Agent {
            name: existing.name.clone(),
            persona: request.persona,
        },
    );

    let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(agent)))
}
