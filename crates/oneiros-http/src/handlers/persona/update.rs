use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<Persona>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(PersonaRequests::SetPersona(persona))?))
}
