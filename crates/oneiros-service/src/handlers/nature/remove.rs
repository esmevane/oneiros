use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<NatureName>,
) -> Result<StatusCode, Error> {
    let event = Events::Nature(NatureEvents::NatureRemoved { name });

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok(StatusCode::OK)
}
