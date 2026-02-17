use axum::{extract::Path, http::StatusCode};
use oneiros_model::TextureName;
use oneiros_protocol::{Events, TextureEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<TextureName>,
) -> Result<StatusCode, Error> {
    let event = Events::Texture(TextureEvents::TextureRemoved { name });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
