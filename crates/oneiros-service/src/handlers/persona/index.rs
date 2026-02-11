use axum::Json;
use oneiros_model::{Description, Persona, PersonaName, Prompt};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Persona>>, Error> {
    let personas = ticket
        .db
        .list_personas()?
        .into_iter()
        .map(|(name, desc, prompt)| Persona {
            name: PersonaName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect::<Vec<_>>();

    Ok(Json(personas))
}
