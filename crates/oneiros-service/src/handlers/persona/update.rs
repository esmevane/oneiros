use axum::{Json, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<PersonaRecord>,
) -> Result<(StatusCode, Json<PersonaRecord>), Error> {
    let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(persona)))
}
