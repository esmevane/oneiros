use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<NatureRecord>,
) -> Result<(StatusCode, Json<NatureRecord>), Error> {
    let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(nature)))
}
