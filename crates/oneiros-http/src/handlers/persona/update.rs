use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<Persona>,
) -> Result<Json<PersonaResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_persona(PersonaRequests::SetPersona(persona))?;

    Ok(Json(response))
}
