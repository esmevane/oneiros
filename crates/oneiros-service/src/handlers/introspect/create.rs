use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<Agent>, Error> {
    let (id, name, persona, desc, prompt) = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let agent = Agent {
        id: id.parse().unwrap_or_default(),
        name: AgentName::new(name),
        persona: PersonaName::new(persona),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    };

    let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&complete, &[])?;

    Ok(Json(agent))
}
