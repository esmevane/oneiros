use axum::{Json, http::StatusCode};
use oneiros_model::{Events, Level, LevelEvents};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<(StatusCode, Json<Level>), Error> {
    let event = Events::Level(LevelEvents::LevelSet(level.clone()));

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    Ok((StatusCode::OK, Json(level)))
}
