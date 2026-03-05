use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<LevelName>,
) -> Result<StatusCode, Error> {
    ticket.service().remove_level(name)?;

    Ok(StatusCode::OK)
}
