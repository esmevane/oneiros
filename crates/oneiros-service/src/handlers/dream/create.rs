use axum::{Json, extract::Path};
use oneiros_model::*;
use oneiros_protocol::{DreamingEvents, Events};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let persona = ticket
        .db
        .get_persona(&agent.persona)?
        .ok_or(NotFound::Persona(agent.persona.clone()))?;

    let memories = ticket.db.list_memories_by_agent(agent.id.to_string())?;
    let cognitions = ticket.db.list_cognitions_by_agent(agent.id.to_string())?;
    let experiences = ticket.db.list_experiences_by_agent(agent.id.to_string())?;
    let textures = ticket.db.list_textures()?;
    let levels = ticket.db.list_levels()?;
    let sensations = ticket.db.list_sensations()?;

    let context = DreamContext {
        agent,
        persona,
        memories,
        cognitions,
        experiences,
        textures,
        levels,
        sensations,
    };

    let complete = Events::Dreaming(DreamingEvents::DreamComplete(Box::new(context.clone())));
    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
