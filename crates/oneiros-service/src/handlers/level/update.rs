use axum::{Json, http::StatusCode};
use oneiros_model::*;
use oneiros_protocol::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<LevelRecord>,
) -> Result<(StatusCode, Json<LevelRecord>), Error> {
    let event = Events::Level(LevelEvents::LevelSet(level.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(level)))
}
