use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(nature): Json<Nature>,
) -> Result<(StatusCode, Json<Nature>), Error> {
    let nature = ticket.service().set_nature(nature)?;

    Ok((StatusCode::OK, Json(nature)))
}
