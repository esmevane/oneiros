use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<Persona>,
) -> Result<Json<PersonaResponses>, Error> {
    let persona = ticket.service().set_persona(persona)?;

    Ok(Json(persona))
}
