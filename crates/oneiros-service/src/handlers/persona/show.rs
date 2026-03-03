use axum::{Json, extract::Path};
use oneiros_model::{Persona, PersonaName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<PersonaName>,
) -> Result<Json<Persona>, Error> {
    let persona = ticket.service().get_persona(&given_name)?;

    Ok(Json(persona))
}
