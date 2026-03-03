use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<(StatusCode, Json<Texture>), Error> {
    let texture = ticket.service().set_texture(texture)?;

    Ok((StatusCode::OK, Json(texture)))
}
