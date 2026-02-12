use axum::{Json, extract::Path};
use oneiros_model::{Agent, AgentName, Description, PersonaName, Prompt};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<AgentName>,
) -> Result<Json<Agent>, Error> {
    let (id, name, persona, desc, prompt) = ticket
        .db
        .get_agent(&given_name)?
        .ok_or(NotFound::Agent(given_name))?;

    Ok(Json(Agent {
        id: id.parse().unwrap_or_default(),
        name: AgentName::new(name),
        persona: PersonaName::new(persona),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    }))
}
