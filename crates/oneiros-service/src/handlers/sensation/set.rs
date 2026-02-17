use axum::{Json, http::StatusCode};
use oneiros_model::{Events, Sensation, SensationEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<Sensation>,
) -> Result<(StatusCode, Json<Sensation>), Error> {
    let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::OK, Json(sensation)))
}
