use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<SensationRecord>,
) -> Result<(StatusCode, Json<SensationRecord>), Error> {
    let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(sensation)))
}
