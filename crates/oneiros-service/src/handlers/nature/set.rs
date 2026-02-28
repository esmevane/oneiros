use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<Nature>,
) -> Result<(StatusCode, Json<Nature>), Error> {
    let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));

    ticket.db.log_event(&event, projections::BRAIN)?;

    Ok((StatusCode::OK, Json(nature)))
}
