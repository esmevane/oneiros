use axum::{Json, extract::Path};
use oneiros_model::{Persona, PersonaName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<PersonaName>,
) -> Result<Json<Persona>, Error> {
    let persona = ticket
        .db
        .get_persona(&given_name)?
        .ok_or(NotFound::Persona(given_name))?;

    Ok(Json(persona))
}
