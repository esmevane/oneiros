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

    let agent = AgentRecord::init(
        request.description,
        request.prompt,
        Agent {
            name: request.name,
            persona: request.persona,
        },
    );

    let emerged = Events::Lifecycle(LifecycleEvents::Emerged {
        name: agent.name.clone(),
    });

    ticket.db.log_event(&emerged, &[])?;

    let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
    ticket.db.log_event(&created, projections::brain::ALL)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
