use axum::{Json, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<(StatusCode, Json<Level>), Error> {
    let event = Events::Level(LevelEvents::LevelSet(level.clone()));

    ticket.db.log_event(&event, projections::brain::ALL)?;

    Ok((StatusCode::OK, Json(level)))
}
