use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Agent>), Error> {
    ticket
        .db
        .get_persona(&request.persona)?
        .ok_or(NotFound::Persona(request.persona.clone()))?;

    if ticket.db.agent_name_exists(&request.name)? {
        return Err(Conflicts::Agent(request.name).into());
    }

    let agent_name = request.name.clone();

    let agent = Agent::init(
        request.description,
        request.prompt,
        request.name,
        request.persona,
    );

    let emerged = Events::Lifecycle(LifecycleEvents::Emerged { name: agent_name });

    ticket.db.log_event(&emerged, &[])?;
    ticket.broadcast(&emerged);

    let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
    ticket.db.log_event(&created, projections::BRAIN)?;
    ticket.broadcast(&created);

    Ok((StatusCode::CREATED, Json(agent)))
}
