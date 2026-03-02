use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<(StatusCode, Json<Texture>), Error> {
    let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));

    ticket.db.log_event(&event, projections::BRAIN)?;
    ticket.broadcast(&event);

    Ok((StatusCode::OK, Json(texture)))
}
