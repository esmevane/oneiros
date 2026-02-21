use axum::{Json, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(request): Json<CreateAgentRequest>,
) -> Result<(StatusCode, Json<Identity<AgentId, Agent>>), Error> {
    ticket
        .db
        .get_persona(&request.persona)?
        .ok_or(NotFound::Persona(Key::Id(request.persona.clone())))?;

    if ticket.db.agent_name_exists(&request.name)? {
        return Err(Conflicts::Agent(request.name).into());
    }

    let agent = Identity::new(
        AgentId::new(),
        Agent {
            name: request.name,
            persona: request.persona,
            description: request.description,
            prompt: request.prompt,
        },
    );

    let emerged = Events::Lifecycle(LifecycleEvents::Emerged {
        name: agent.name.clone(),
    });
    ticket.db.log_event(&emerged, &[])?;

    let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
    ticket
        .db
        .log_event(&created, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::CREATED, Json(agent)))
}
