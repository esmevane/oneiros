use axum::{Json, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<TextureRecord>,
) -> Result<(StatusCode, Json<TextureRecord>), Error> {
    let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(texture)))
}
