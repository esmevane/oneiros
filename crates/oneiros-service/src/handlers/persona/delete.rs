use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<PersonaName>,
) -> Result<StatusCode, Error> {
    let event = Events::Persona(PersonaEvents::PersonaRemoved { name });

    ticket.db.log_event(&event, projections::BRAIN)?;
    ticket.broadcast(&event);

    Ok(StatusCode::OK)
}
