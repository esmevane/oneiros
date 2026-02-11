use axum::{extract::Path, http::StatusCode};
use oneiros_model::{Events, PersonaEvents, PersonaName, projections};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<PersonaName>,
) -> Result<StatusCode, Error> {
    let event = Events::Persona(PersonaEvents::PersonaRemoved { name });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
