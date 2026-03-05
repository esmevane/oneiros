use axum::Json;
use oneiros_model::PersonaResponses;

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<PersonaResponses>, Error> {
    let personas = ticket.service().list_personas()?;

    Ok(Json(personas))
}
