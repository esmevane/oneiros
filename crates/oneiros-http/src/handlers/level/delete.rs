use axum::{extract::Path, http::StatusCode};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(name): Path<LevelName>,
) -> Result<StatusCode, Error> {
    ticket
        .service()
        .dispatch_level(LevelRequests::RemoveLevel(RemoveLevelRequest { name }))?;

    Ok(StatusCode::OK)
}
