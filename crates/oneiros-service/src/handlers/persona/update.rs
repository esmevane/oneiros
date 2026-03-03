use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(persona): Json<Persona>,
) -> Result<(StatusCode, Json<Persona>), Error> {
    let persona = ticket.service().set_persona(persona)?;

    Ok((StatusCode::OK, Json(persona)))
}
