use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(sensation): Json<Sensation>,
) -> Result<(StatusCode, Json<Sensation>), Error> {
    let sensation = ticket.service().set_sensation(sensation)?;

    Ok((StatusCode::OK, Json(sensation)))
}
