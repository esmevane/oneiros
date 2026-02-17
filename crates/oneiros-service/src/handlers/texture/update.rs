use axum::{Json, http::StatusCode};
use oneiros_model::{Events, Texture, TextureEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(texture): Json<Texture>,
) -> Result<(StatusCode, Json<Texture>), Error> {
    let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::OK, Json(texture)))
}
