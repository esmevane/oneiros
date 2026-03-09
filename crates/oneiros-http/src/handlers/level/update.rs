use axum::Json;
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Json(level): Json<Level>,
) -> Result<Json<LevelResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_level(LevelRequests::SetLevel(level))?;

    Ok(Json(response))
}
