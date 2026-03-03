use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<(StatusCode, Json<Level>), Error> {
    let level = ticket.service().set_level(level)?;

    Ok((StatusCode::OK, Json(level)))
}
