use axum::{extract::Path, http::StatusCode};
use oneiros_model::LevelName;
use oneiros_protocol::{Events, LevelEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<LevelName>,
) -> Result<StatusCode, Error> {
    let event = Events::Level(LevelEvents::LevelRemoved { name });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok(StatusCode::OK)
}
