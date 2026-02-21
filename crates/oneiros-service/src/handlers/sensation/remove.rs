use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<SensationName>,
) -> Result<StatusCode, Error> {
    let event = Events::Sensation(SensationEvents::SensationRemoved { name });

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok(StatusCode::OK)
}
