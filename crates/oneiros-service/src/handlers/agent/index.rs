use axum::Json;
use oneiros_model::{Agent, AgentName, Description, PersonaName, Prompt};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Agent>>, Error> {
    let agents = ticket
        .db
        .list_agents()?
        .into_iter()
        .map(|(id, name, persona, desc, prompt)| Agent {
            id: id.parse().unwrap_or_default(),
            name: AgentName::new(name),
            persona: PersonaName::new(persona),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect::<Vec<_>>();

    Ok(Json(agents))
}
