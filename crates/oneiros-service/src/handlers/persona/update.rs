use axum::{Json, http::StatusCode};
use oneiros_model::{Events, Persona, PersonaEvents, projections};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<Persona>,
) -> Result<(StatusCode, Json<Persona>), Error> {
    let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::OK, Json(persona)))
}
