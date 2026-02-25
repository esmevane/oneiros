use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<AgentRecord>), Error> {
    ticket
        .db
        .get_persona(&request.persona)?
        .ok_or(NotFound::Persona(request.persona.clone()))?;

    if ticket.db.agent_name_exists(&request.name)? {
        return Err(Conflicts::Agent(request.name).into());
    }

    let agent = Agent {
        name: request.name,
        persona: request.persona,
    };

    let agent_name = agent.name.clone();

    let record = AgentRecord::init(request.description, request.prompt, agent);

    let emerged = Events::Lifecycle(LifecycleEvents::Emerged { name: agent_name });

    ticket.db.log_event(&emerged, &[])?;

    let created = Events::Agent(AgentEvents::AgentCreated(record.clone()));
    ticket.db.log_event(&created, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(record)))
}
