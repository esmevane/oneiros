use axum::{Json, extract::Path};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(Key::Id(agent_name.clone())))?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let persona = ticket
        .db
        .get_persona(&agent.persona)?
        .ok_or(NotFound::Persona(Key::Id(agent.persona.clone())))?;

    let memories = ticket.db.list_memories_by_agent(&agent.id)?;
    let cognitions = ticket.db.list_cognitions_by_agent(&agent.id)?;
    let experiences = ticket.db.list_experiences_by_agent(&agent.id)?;
    let connections = ticket.db.list_connections()?;
    let textures = ticket.db.list_textures()?;
    let levels = ticket.db.list_levels()?;
    let sensations = ticket.db.list_sensations()?;
    let natures = ticket.db.list_natures()?;

    let context = DreamContext {
        agent,
        persona,
        memories,
        cognitions,
        experiences,
        connections,
        textures,
        levels,
        sensations,
        natures,
    };

    let complete = Events::Dreaming(DreamingEvents::DreamComplete(Box::new(context.clone())));
    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
