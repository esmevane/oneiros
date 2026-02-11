use axum::{Json, extract::Path};
use oneiros_model::{Description, Persona, PersonaName, Prompt};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<PersonaName>,
) -> Result<Json<Persona>, Error> {
    let (name, desc, prompt) = ticket
        .db
        .get_persona(&given_name)?
        .ok_or(NotFound::Persona(given_name))?;

    Ok(Json(Persona {
        name: PersonaName::new(name),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    }))
}
