use axum::{extract::Path, http::StatusCode};
use oneiros_model::{Events, SensationEvents, SensationName, projections};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<SensationName>,
) -> Result<StatusCode, Error> {
    let event = Events::Sensation(SensationEvents::SensationRemoved { name });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
